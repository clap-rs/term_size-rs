use winapi::shared::minwindef::DWORD;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use winapi::um::wincon::GetConsoleScreenBufferInfo;
use winapi::um::wincon::{CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT};

/// Calls GetConsoleScreenBufferInfo. Returns None if all of the streams are
/// not to a terminal, or there is an error.
fn get_dimensions_any() -> Option<(usize, usize)> {
    let null_coord = COORD { X: 0, Y: 0 };
    let null_smallrect = SMALL_RECT {
        Left: 0,
        Top: 0,
        Right: 0,
        Bottom: 0,
    };

    let mut console_data = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: null_coord,
        dwCursorPosition: null_coord,
        wAttributes: 0,
        srWindow: null_smallrect,
        dwMaximumWindowSize: null_coord,
    };

    if unsafe { GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &mut console_data) } != 0 ||
        unsafe { GetConsoleScreenBufferInfo(GetStdHandle(STD_INPUT_HANDLE), &mut console_data) } != 0 ||
        unsafe { GetConsoleScreenBufferInfo(GetStdHandle(STD_ERROR_HANDLE), &mut console_data) } != 0 {
        Some(((console_data.srWindow.Right - console_data.srWindow.Left + 1) as usize,
                (console_data.srWindow.Bottom - console_data.srWindow.Top + 1) as usize))
    } else {
        None
    }
}

/// Calls GetConsoleScreenBufferInfo. Returns None if the stream is not to a
/// terminal, or there is an error.
fn get_dimensions(hdl: DWORD) -> Option<(usize, usize)> {
    let null_coord = COORD { X: 0, Y: 0 };
    let null_smallrect = SMALL_RECT {
        Left: 0,
        Top: 0,
        Right: 0,
        Bottom: 0,
    };

    let mut console_data = CONSOLE_SCREEN_BUFFER_INFO {
        dwSize: null_coord,
        dwCursorPosition: null_coord,
        wAttributes: 0,
        srWindow: null_smallrect,
        dwMaximumWindowSize: null_coord,
    };

    if unsafe { GetConsoleScreenBufferInfo(GetStdHandle(hdl), &mut console_data) } != 0 {
        Some(((console_data.srWindow.Right - console_data.srWindow.Left + 1) as usize,
                (console_data.srWindow.Bottom - console_data.srWindow.Top + 1) as usize))
    } else {
        None
    }
}

/// Query the current processes's output (`stdout`), input (`stdin`), and
/// error (`stderr`) in that order, returning its width and height as a
/// number of characters.
///
/// # Errors
///
/// Returns `None` if the output isn't to a terminal.
///
/// # Example
///
/// To get the dimensions of your terminal window, simply use the following:
///
/// ```no_run
/// # use term_size;
/// if let Some((w, h)) = term_size::dimensions() {
///     println!("Width: {}\nHeight: {}", w, h);
/// } else {
///     println!("Unable to get term size :(")
/// }
/// ```
pub fn dimensions() -> Option<(usize, usize)> {
    get_dimensions_any()
}

/// Query the current processes's output (`stdout`) *only*, returning its
/// width and height as a number of characters. Returns `None` if the output
/// isn't to a terminal.
///
/// # Errors
///
/// Returns `None` if the output isn't to a terminal.
///
/// # Example
///
/// To get the dimensions of your terminal window, simply use the following:
///
/// ```no_run
/// # use term_size;
/// if let Some((w, h)) = term_size::dimensions() {
///     println!("Width: {}\nHeight: {}", w, h);
/// } else {
///     println!("Unable to get term size :(")
/// }
/// ```
pub fn dimensions_stdout() -> Option<(usize, usize)> {
    get_dimensions(STD_OUTPUT_HANDLE)
}

/// Query the current processes's input (`stdin`) *only*, returning its width
/// and height as a number of characters. Returns `None` if the output isn't
/// to a terminal.
///
/// # Errors
///
/// Returns `None` if the output isn't to a terminal.
///
/// # Example
///
/// To get the dimensions of your terminal window, simply use the following:
///
/// ```no_run
/// # use term_size;
/// if let Some((w, h)) = term_size::dimensions() {
///     println!("Width: {}\nHeight: {}", w, h);
/// } else {
///     println!("Unable to get term size :(")
/// }
/// ```
pub fn dimensions_stdin() -> Option<(usize, usize)> {
    get_dimensions(STD_INPUT_HANDLE)
}

/// Query the current processes's error output (`stderr`) *only*, returning
/// its width and height as a number of characters. Returns `None` if the
/// output isn't to a terminal.
///
/// # Errors
///
/// Returns `None` if the output isn't to a terminal.
///
/// # Example
///
/// To get the dimensions of your terminal window, simply use the following:
///
/// ```no_run
/// # use term_size;
/// if let Some((w, h)) = term_size::dimensions() {
///     println!("Width: {}\nHeight: {}", w, h);
/// } else {
///     println!("Unable to get term size :(")
/// }
/// ```
pub fn dimensions_stderr() -> Option<(usize, usize)> {
    get_dimensions(STD_ERROR_HANDLE)
}
