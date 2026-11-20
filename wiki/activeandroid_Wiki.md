# SigmaOS-activeandroid Component Wiki

## Overview
SigmaOS-activeandroid provides Android compatibility layer for the SigmaOS operating system. It demonstrates the Android runtime integration and application compatibility within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic Android initialization.

## Principles
- Single Responsibility
- Android Compatibility Abstraction
- Fail-Fast on Android Initialization Errors
- Deterministic Android Setup
- Clear Abstraction Over Android APIs
- Composition over Inheritance for Android extensions

## Architecture
SigmaOS-activeandroid follows an Android compatibility pattern:
- **Android Core**: Provides Android runtime emulation, API translation, and JNI bindings
- **Application Framework**: Supports Android application component lifecycle and resource management
- **System Integration**: Handles Android system services and native library loading
- **Error Handling**: Fail-fast on invalid Android configurations or unsupported API levels

## API
The component provides the following public APIs:
- `android::api::call(interface, method, args)` - Call an Android API method
- `android::activity::launch(intent)` - Launch an Android activity
- `android::package::install(path)` - Install an Android package
- `android::status()` - Get Android compatibility status and version information
- Error types for Android failure scenarios or unsupported API levels

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific Android configuration
2. **Act**: Execute the code under test (Android API call, activity launch)
3. **Assert**: Verify the expected Android results

Example:
```rust
#[test]
fn test_android_activity_launch_aaa() {
    // Arrange
    let intent = "android.intent.action.VIEW";
    
    // Act
    let result = android::activity::launch(intent);
    
    // Assert
    assert!(result.is_ok());
    assert!(result.unwrap().is_running());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Android initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all Android failure scenarios
- Unit tests provided for API calls and activity management
- Integration tests for full Android compatibility with actual Android applications

## TODO
- Add support for additional Android API levels and system versions
- Expand application framework capabilities (activity, service, broadcast receiver lifecycle)
- Add native library loading and JNI binding features
- Create integration tests with actual Android application execution
- Implement Android preference and settings management