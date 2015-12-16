//! [2.3.3 Information About the History List](https://goo.gl/8OWMTy)
//!
//! These functions return information about the entire history list or individual list entries.
use libc::c_int;
use history::{HistoryEntry, vars};
use time::Timespec;

mod ext_listinfo {
    use libc::{c_int, c_long};
    use history::HistoryEntry;

    extern "C" {
        pub fn history_list() -> *mut *mut HistoryEntry;
        pub fn where_history() -> c_int;
        pub fn current_history() -> *mut HistoryEntry;
        pub fn history_get(which: c_int) -> *mut HistoryEntry;
        pub fn history_get_time(which: *mut HistoryEntry) -> c_long;
        pub fn history_total_bytes() -> c_int;
    }
}

/// Return a Vec<HistoryEntry> which is the current input history. Element 0 of this list is the
/// beginning of time. If there is no history, return an empty vector.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listinfo, listmgmt};
///
/// assert!(listmgmt::add("test").is_ok());
/// let entries = listinfo::list().unwrap();
/// assert!(entries.len() == 1);
/// ```
pub fn list() -> Result<Vec<HistoryEntry>, ::HistoryError> {
    ::history::mgmt::init();
    unsafe {
        let ptr = &mut *ext_listinfo::history_list();

        if ptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to access history list"))
        } else {
            let len = vars::history_length;
            let mut entries = Vec::new();
            for i in 0..len {
                let entry = *ptr.offset(i as isize);
                entries.push(entry);
            }
            Ok(entries)
        }
    }
}

/// Returns the offset of the current history element.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listinfo, listmgmt};
///
/// assert!(listmgmt::add("test").is_ok());
/// assert!(listinfo::offset() == 0);
/// ```
pub fn offset() -> usize {
    ::history::mgmt::init();
    unsafe { ext_listinfo::where_history() as usize }
}

/// Return the history entry at the current position, as determined by `where_history()``. If there
/// is no entry there, return a HistoryError.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listinfo, listmgmt};
///
/// assert!(listmgmt::add("test").is_ok());
/// assert!(listinfo::current().is_ok());
/// ```
pub fn current<'a>() -> Result<&'a mut HistoryEntry, ::HistoryError> {
    ::history::mgmt::init();
    unsafe {
        let ptr = ext_listinfo::current_history();

        if ptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to read the current history!"))
        } else {
            Ok(&mut *ptr)
        }
    }
}

/// Return the history entry at position offset, starting from `history_base`. If there is no entry
/// there, or if offset is greater than the history length, return a HistoryError.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listinfo, listmgmt, vars};
///
/// assert!(listmgmt::add("test").is_ok());
/// assert!(vars::history_base == 1);
/// assert!(listinfo::get(1).is_ok());
/// ```
pub fn get<'a>(offset: usize) -> Result<&'a mut HistoryEntry, ::HistoryError> {
    ::history::mgmt::init();
    unsafe {
        let ptr = ext_listinfo::history_get(offset as c_int);

        if ptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to get the history!"))
        } else {
            Ok(&mut *ptr)
        }
    }
}

/// Return the time stamp associated with the history entry entry.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listinfo, listmgmt, vars};
///
/// vars::set_comment_char(':');
/// assert!(listmgmt::add("test").is_ok());
/// assert!(vars::history_base == 1);
/// let entry = listinfo::get(1).unwrap();
/// assert!(listinfo::get_time(entry).sec > 0);
/// ```
pub fn get_time<'a>(entry: &'a mut HistoryEntry) -> Timespec {
    ::history::mgmt::init();
    Timespec::new(unsafe { ext_listinfo::history_get_time(entry) } as i64, 0)
}

/// Return the number of bytes that the primary history entries are using. This function returns the
/// sum of the lengths of all the lines in the history.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listinfo, listmgmt, vars};
///
/// assert!(listmgmt::add("test").is_ok());
/// assert!(vars::history_base == 1);
/// assert!(listinfo::total_bytes() > 0);
/// ```
pub fn total_bytes() -> usize {
    ::history::mgmt::init();
    unsafe { ext_listinfo::history_total_bytes() as usize }
}

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn test_history_list() {
        // ::history::mgmt::init();
        // ::history::listmgmt::clear();
        // assert!(::history::listmgmt::add("ls -al").is_ok());
        // assert!(::history::listmgmt::add("test").is_ok());
        // let list = list().unwrap();
        // assert_eq!(list.len(), 2);
    }
}
