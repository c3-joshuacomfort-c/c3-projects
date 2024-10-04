import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--grid", help="Specify --grid to create the boiler plate for UiSdlDataGrid", action='store_true')
    parser.add_argument("--filter", help="Specify --filter to create the boiler plate for UiSdlFilterPanel", action='store_true')
    
    args = vars(parser.parse_args())
    
    for arg in args:
        print(arg, args[arg])