# Check if running as root
if [ "$(id -u)" != "0" ]; then
    echo "This script must be run as root" 1>&2
    exit 1
fi

clear

echo "Cleaning up..."
rm -rf /etc/stable-os/flags

echo "Preparing..."
cargo run

echo "Adding test flags..."
touch /etc/stable-os/flags/bool1
touch /etc/stable-os/flags/bool2
touch /etc/stable-os/flags/bool3
echo "1" > /etc/stable-os/flags/num1
echo "2" > /etc/stable-os/flags/num2
echo "3" > /etc/stable-os/flags/num3

echo "Running flagd..."
cargo run

echo "Updating test flags..."
rm /etc/stable-os/flags/bool1
rm /etc/stable-os/flags/bool2
echo "4" > /etc/stable-os/flags/num1
echo "5" > /etc/stable-os/flags/num2
rm /etc/stable-os/flags/num3

echo "Running flagd..."
cargo run

echo "Done!"