def unpack_hex(hex_string):
    full_int = int(hex_string, 16)
    
    mask32 = (1 << 32) - 1
    mask112 = (1 << 112) - 1
    
    segment1 = full_int & mask112
    segment2 = (full_int >> 112) & mask112
    segment3 = (full_int >> 144) & mask32

    return segment1, segment2, segment3

# Example hexadecimal input
hex_input = "0x6639294c0000000000000000000000030b4f0000000000000000000000061e68"  # Replace with your actual hex string
result = unpack_hex(hex_input)
print(f"Segment 1: {result[0]}")
print(f"Segment 2: {result[1]}")
print(f"Segment 3: {result[2]}")
