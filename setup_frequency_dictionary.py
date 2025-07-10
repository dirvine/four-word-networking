#!/usr/bin/env python3
"""
Download and process Hugging Face common-words-79k dataset for three-word-networking
"""

import sys
import os

def main():
    print("=== Hugging Face Common Words Dataset Downloader ===\n")
    
    # Check for required libraries
    try:
        from datasets import load_dataset
        import pandas as pd
        print("✓ Required libraries found")
    except ImportError as e:
        print("✗ Missing required libraries")
        print("\nPlease install them with:")
        print("  pip3 install datasets pandas")
        print("\nOr if using conda:")
        print("  conda install -c huggingface datasets pandas")
        return 1
    
    try:
        # Create data directory if it doesn't exist
        os.makedirs("data", exist_ok=True)
        
        print("\nDownloading common-words-79k dataset from Hugging Face...")
        print("This may take a few moments...\n")
        
        # Download the dataset
        dataset = load_dataset("jaagli/common-words-79k", split="whole")
        
        print(f"✓ Downloaded {len(dataset)} entries")
        
        # Convert to pandas DataFrame
        df = dataset.to_pandas()
        
        # Display dataset info
        print(f"\nDataset columns: {df.columns.tolist()}")
        print(f"Dataset shape: {df.shape}")
        
        # Save raw dataset
        df.to_csv("data/common-words-79k-raw.csv", index=False)
        print("\n✓ Saved raw dataset to data/common-words-79k-raw.csv")
        
        # Process and save word list
        if 'alias' in df.columns:
            # Extract words and clean them
            words_df = pd.DataFrame()
            words_df['word'] = df['alias'].str.lower().str.strip()
            
            # Add frequency if available
            if 'frequency' in df.columns:
                words_df['frequency'] = df['frequency']
                words_df = words_df.sort_values('frequency', ascending=False)
            
            # Remove duplicates
            words_df = words_df.drop_duplicates(subset=['word'])
            
            # Save processed words
            words_df.to_csv("data/common-words-processed.csv", index=False)
            print(f"✓ Saved {len(words_df)} unique words to data/common-words-processed.csv")
            
            # Show sample
            print("\nSample words from dataset:")
            for i, row in words_df.head(20).iterrows():
                if 'frequency' in words_df.columns:
                    print(f"  {row['word']} (frequency: {row['frequency']})")
                else:
                    print(f"  {row['word']}")
        else:
            print("\n⚠ Warning: 'alias' column not found in dataset")
            print("Available columns:", df.columns.tolist())
            
            # Try to extract words from first text column
            text_cols = [col for col in df.columns if df[col].dtype == 'object']
            if text_cols:
                print(f"\nExtracting from column: {text_cols[0]}")
                words = df[text_cols[0]].str.lower().str.strip().unique()
                pd.DataFrame({'word': words}).to_csv("data/common-words-processed.csv", index=False)
                print(f"✓ Saved {len(words)} unique words")
        
        print("\n✓ Dataset download complete!")
        print("\nNext steps:")
        print("1. Run: cargo run --bin process_frequency_dictionary")
        print("2. This will filter and prepare the dictionary for use")
        
    except Exception as e:
        print(f"\n✗ Error downloading dataset: {e}")
        print("\nTroubleshooting:")
        print("1. Check your internet connection")
        print("2. Ensure you have access to Hugging Face datasets")
        print("3. Try updating datasets library: pip3 install --upgrade datasets")
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())