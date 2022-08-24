import os, sys

def run_npm_script(script: str):
    os.chdir("frontend")
    print(os.getcwd())
    os.execvp("npm", ["npm", "run", script])

if __name__ == "__main__":
    assert len(sys.argv) == 2
    mode = sys.argv[1]

    assert mode in ("build", "dev")
    run_npm_script(mode)
