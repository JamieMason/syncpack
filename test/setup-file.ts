// Disable chalk so that ANSI colors are not output in the test output.
process.env.FORCE_COLOR = '0';

// Make file paths deterministic
process.env.MOCK_CWD = '/fake/dir';
