IgorPlugUDP Plugin
===============

Overview
--------
The IgorPlugUDP plugin enables integration with Igor Češko's UDP IR receiver, a network-connected device that receives infrared signals from standard remote controls and broadcasts them over UDP. This allows multiple computers on the network to receive and process IR commands simultaneously.

Core Components
--------------

UDP Receiver
~~~~~~~~~~
- Asynchronous UDP packet handling
- Network socket management
- Packet validation and processing
- IR command decoding

IR Decoder
~~~~~~~~~
- IR protocol interpretation
- Command sequence handling
- Signal validation
- Event generation

Network Manager
~~~~~~~~~~~~
- UDP broadcast reception
- Socket configuration
- Address management
- Port binding

Key Features
-----------

Network Features
~~~~~~~~~~~~~
- UDP broadcast support
- Configurable IP address
- Port customization
- Multi-computer reception

IR Processing
~~~~~~~~~~~
- Standard IR protocol support
- Command sequence handling
- Error detection
- Event triggering

Configuration
~~~~~~~~~~~
- Event prefix customization
- Network settings management
- Port configuration
- Address specification

Migration Considerations
----------------------

Core Dependencies
~~~~~~~~~~~~~~
- Replace asyncore with tokio
- Implement async UDP handling
- Use structured error types
- Add strong typing for commands

Network Compatibility
~~~~~~~~~~~~~~~~~
- Maintain UDP broadcast support
- Preserve packet structure
- Support existing IR protocols
- Handle network configuration

Implementation Strategy
---------------------

UDP Handler
~~~~~~~~~
.. code-block:: rust

    pub struct UdpReceiver {
        address: SocketAddr,
        port: u16,
        socket: UdpSocket,
        ir_command: Vec<u8>,
    }

    impl UdpReceiver {
        pub async fn start(&mut self) -> Result<()> {
            // Bind socket
            // Configure broadcast
            // Start receive loop
            // Handle errors
        }
        
        pub async fn process_packet(&mut self, data: &[u8]) -> Result<()> {
            // Validate packet
            // Extract command
            // Process sequence
            // Generate event
        }
    }

IR Command Processing
~~~~~~~~~~~~~~~~~~
.. code-block:: rust

    pub struct IrDecoder {
        command_buffer: Vec<u8>,
        decoder: IrProtocolDecoder,
        event_dispatcher: EventDispatcher,
    }

    impl IrDecoder {
        pub fn decode_command(&mut self, data: &[u8], length: usize) -> Result<()> {
            // Validate data
            // Process command
            // Handle sequence
            // Trigger event
        }
        
        pub fn handle_error(&self, error_type: ErrorType) {
            // Log error
            // Trigger event
            // Reset state
            // Notify system
        }
    }

Network Management
~~~~~~~~~~~~~~~
.. code-block:: rust

    pub struct NetworkManager {
        socket: UdpSocket,
        config: NetworkConfig,
        error_handler: ErrorHandler,
    }

    impl NetworkManager {
        pub async fn configure(&mut self) -> Result<()> {
            // Set up socket
            // Configure broadcast
            // Bind address
            // Handle errors
        }
        
        pub async fn handle_connection(&mut self) -> Result<()> {
            // Monitor connection
            // Process packets
            // Handle timeouts
            // Manage errors
        }
    }

Testing Strategy
---------------

Unit Tests
~~~~~~~~~
- UDP packet handling
- IR command decoding
- Network configuration
- Error handling

Integration Tests
~~~~~~~~~~~~~~~
- Network communication
- IR signal processing
- Event generation
- Configuration management

Protocol Tests
~~~~~~~~~~~~
- UDP packet validation
- IR protocol compliance
- Command sequence handling
- Error detection

Error Handling
-------------

Network Errors
~~~~~~~~~~~
- Socket binding failures
- UDP reception errors
- Broadcast issues
- Configuration problems

Protocol Errors
~~~~~~~~~~~~
- Invalid packets
- Malformed commands
- Sequence errors
- Validation failures

IR Errors
~~~~~~~
- Signal errors
- Command errors
- Sequence failures
- Decoding issues

Platform Considerations
---------------------

Windows Integration
~~~~~~~~~~~~~~~~
- UDP socket configuration
- Network interface handling
- Broadcast support
- Resource management

Cross-Platform Support
~~~~~~~~~~~~~~~~~~~
- Network abstraction
- Socket compatibility
- Error standardization
- Resource handling 