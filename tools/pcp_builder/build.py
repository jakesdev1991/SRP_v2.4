import tarfile
import os
import json
def build_pcp(output_path):
    print(f"Building PCP at {output_path}...")
    with open(output_path, 'wb') as f_out:
        f_out.write(b"PCP_v2.4_HEADER\n")
if __name__ == "__main__":
    pass
