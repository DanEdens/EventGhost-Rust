OnkyoISCP Plugin
==============

Overview
--------
The OnkyoISCP plugin provides control capabilities for Onkyo receivers that support the Integra Serial Control Protocol (ISCP). It enables bidirectional communication over TCP/IP, allowing both command sending and event reception from Onkyo devices.

Core Components
--------------

ISCP Protocol Handler
~~~~~~~~~~~~~~~~~~
- Implements the ISCP protocol specification
- Handles packet formation and parsing
- Manages protocol versioning
- Supports receiver-specific commands

Connection Manager
~~~~~~~~~~~~~~~
- TCP/IP socket management
- Connection state monitoring
- Automatic reconnection handling
- Timeout management

Event System
~~~~~~~~~~
- Asynchronous event reception
- Command response processing
- Event triggering based on receiver state
- Error handling and reporting

Key Features
-----------

Protocol Support
~~~~~~~~~~~~~
- ISCP protocol version 1
- Binary packet handling
- Command queueing
- Response parsing

Connection Features
~~~~~~~~~~~~~~~~
- TCP/IP communication
- Configurable timeout
- Connection recovery
- Socket reuse support

Command System
~~~~~~~~~~~~
- Direct command sending
- Response monitoring
- Error recovery
- Command retries

Migration Considerations
----------------------

Core Dependencies
~~~~~~~~~~~~~~
- Replace socket operations with tokio
- Implement async I/O operations
- Use structured error handling
- Add strong typing for commands

Protocol Compatibility
~~~~~~~~~~~~~~~~~~~
- Maintain ISCP packet structure
- Preserve header formatting
- Support existing command codes
- Handle receiver variations

Implementation Strategy
---------------------

Protocol Implementation
~~~~~~~~~~~~~~~~~~~~
.. code-block:: rust

    pub struct ISCPProtocol {
        header: &'static str,
        header_size: u32,
        version: u8,
        unit_type: u8,
    }

    impl ISCPProtocol {
        pub fn create_packet(&self, command: &str) -> Result<Vec<u8>> {
            // Format ISCP packet
            // Add header
            // Calculate sizes
            // Pack data
        }
        
        pub fn parse_packet(&self, data: &[u8]) -> Result<ISCPMessage> {
            // Validate header
            // Extract message
            // Parse command
            // Handle parameters
        }
    }

Connection Management
~~~~~~~~~~~~~~~~~~
.. code-block:: rust

    pub struct OnkyoConnection {
        address: SocketAddr,
        timeout: Duration,
        socket: TcpStream,
        protocol: ISCPProtocol,
    }

    impl OnkyoConnection {
        pub async fn connect(&mut self) -> Result<()> {
            // Establish connection
            // Configure socket
            // Start receive loop
            // Monitor connection state
        }
        
        pub async fn send_command(&mut self, command: &str) -> Result<()> {
            // Create packet
            // Send data
            // Handle errors
            // Attempt reconnection
        }
    }

Event Handler
~~~~~~~~~~~
.. code-block:: rust

    pub struct EventHandler {
        receiver: mpsc::Receiver<ISCPMessage>,
        event_dispatcher: EventDispatcher,
    }

    impl EventHandler {
        pub async fn process_events(&mut self) {
            // Receive messages
            // Parse events
            // Trigger callbacks
            // Handle errors
        }
        
        pub fn trigger_event(&self, command: &str, parameter: &str) {
            // Format event
            // Dispatch to system
            // Log activity
        }
    }

Testing Strategy
---------------

Unit Tests
~~~~~~~~~
- Protocol packet formation
- Message parsing
- Command formatting
- Error handling

Integration Tests
~~~~~~~~~~~~~~~
- Connection management
- Command sending
- Event reception
- Recovery mechanisms

Protocol Tests
~~~~~~~~~~~~
- ISCP compliance
- Packet validation
- Version handling
- Command responses

Error Handling
-------------

Connection Errors
~~~~~~~~~~~~~~
- Socket timeouts
- Connection drops
- Network failures
- Reconnection logic

Protocol Errors
~~~~~~~~~~~~~
- Invalid packets
- Version mismatches
- Malformed commands
- Response parsing

Command Errors
~~~~~~~~~~~~
- Invalid commands
- Failed transmissions
- Response timeouts
- State inconsistencies

Platform Considerations
---------------------

Windows Integration
~~~~~~~~~~~~~~~~
- Socket configuration
- Network interface handling
- Error code mapping
- Resource cleanup

Cross-Platform Support
~~~~~~~~~~~~~~~~~~~
- Network abstraction
- Error standardization
- Timeout handling
- Resource management 