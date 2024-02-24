installed_path="/usr/bin/conv"

if [[ ! -e $installed_path ]]; then 
    echo "conv is not installed at $installed_path"
    exit 2
fi

version=$(conv --version)
sudo rm $installed_path && echo -e "$version successfully uninstalled"