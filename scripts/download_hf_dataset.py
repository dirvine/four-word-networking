
import sys
try:
    from datasets import load_dataset
    import pandas as pd
    
    print("Downloading common-words-79k dataset from Hugging Face...")
    dataset = load_dataset("jaagli/common-words-79k", split="whole")
    
    print(f"Downloaded {len(dataset)} entries")
    
    # Convert to pandas DataFrame
    df = dataset.to_pandas()
    
    # Save as CSV
    df.to_csv("data/common-words-79k-raw.csv", index=False)
    print("Saved raw dataset to data/common-words-79k-raw.csv")
    
    # Also save just the words with frequencies
    if 'alias' in df.columns and 'frequency' in df.columns:
        words_df = df[['alias', 'frequency']].copy()
        words_df.columns = ['word', 'frequency']
        words_df = words_df.sort_values('frequency', ascending=False)
        words_df.to_csv("data/common-words-frequency.csv", index=False)
        print(f"Saved {len(words_df)} words with frequencies")
    else:
        print("Warning: Expected columns not found. Columns are:", df.columns.tolist())
        
except ImportError as e:
    print(f"Error: Missing required Python library: {e}")
    print("Please install: pip3 install datasets pandas")
    sys.exit(1)
except Exception as e:
    print(f"Error downloading dataset: {e}")
    sys.exit(1)
