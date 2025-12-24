import dbus
import sys
import os

def main():
    try:
        session_bus = dbus.SessionBus()
    except Exception as e:
        print(f"Error connecting to SessionBus: {e}")
        sys.exit(1)

    # Try to get AT-SPI bus address
    atspi_bus_address = os.environ.get('AT_SPI_BUS_ADDRESS')

    if not atspi_bus_address:
        try:
            a11y_bus_obj = session_bus.get_object('org.a11y.Bus', '/org/a11y/bus')
            a11y_bus_iface = dbus.Interface(a11y_bus_obj, 'org.a11y.Bus')
            atspi_bus_address = a11y_bus_iface.GetAddress()
            print(f"Got AT-SPI bus address from session bus: {atspi_bus_address}")
        except Exception as e:
            print(f"Could not get AT-SPI bus address from org.a11y.Bus: {e}")

    if not atspi_bus_address:
        print("Could not determine AT-SPI bus address.")
        sys.exit(1)

    try:
        bus = dbus.bus.BusConnection(atspi_bus_address)
    except Exception as e:
        print(f"Error connecting to AT-SPI bus at {atspi_bus_address}: {e}")
        sys.exit(1)

    dest = 'org.a11y.atspi.Registry'
    path = '/org/a11y/atspi/registry/deviceeventcontroller'
    interface = 'org.a11y.atspi.DeviceEventController'

    try:
        obj = bus.get_object(dest, path)
    except Exception as e:
        print(f"Error getting object {path} from {dest}: {e}")
        sys.exit(1)

    # Introspection
    print(f"--- Introspection for {path} ---")
    try:
        intro_iface = dbus.Interface(obj, 'org.freedesktop.DBus.Introspectable')
        xml = intro_iface.Introspect()
        print(xml)
    except Exception as e:
        print(f"Error introspecting: {e}")
    print("--------------------------------")

    dec_iface = dbus.Interface(obj, interface)
    
    listener_path = "/org/test/listener"
    
    # keys: a(iisi)
    keys = dbus.Array([], signature='(iisi)')
    
    mask = dbus.UInt32(0)
    
    # event_types: [0, 1]
    event_types = [0, 1]
    
    # mode: (bbb)
    mode_arg = (True, True, False) 
    
    for event_type in event_types:
        print(f"Attempting to register listener for event_type={event_type}...")
        try:
            res = dec_iface.RegisterKeystrokeListener(
                dbus.ObjectPath(listener_path),
                keys,
                mask,
                dbus.UInt32(event_type),
                mode_arg
            )
            print(f"Success! Result: {res}")
        except Exception as e:
            print(f"Failed: {e}")

if __name__ == '__main__':
    main()
