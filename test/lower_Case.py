import argparse

# Set up command-line argument parser
parser = argparse.ArgumentParser(description='Convert all words in an input file to lowercase and write to an output file.')
parser.add_argument('input_file', type=str, help='Path to the input file')
parser.add_argument('output_file', type=str, help='Path to the output file')

# Parse arguments
args = parser.parse_args()

# Read the content from the input file
with open(args.input_file, 'r') as file:
    content = file.read()

# Split the content into words (handles spaces, newlines, etc.)
words = content.split()

# Convert each word to lowercase
lowercase_words = [word.lower() for word in words]

# Write the lowercase words to the output file, one per line
with open(args.output_file, 'w') as file:
    for word in lowercase_words:
        file.write(word + '\n')

print(f"Processed {len(words)} words and wrote to {args.output_file}")
