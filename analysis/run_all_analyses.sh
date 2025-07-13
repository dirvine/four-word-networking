#!/bin/bash
# run_all_analyses.sh
#
# Runs all dictionary analysis scripts in sequence.
# This script generates the complete analysis report shown in ../3_word_analysis.md

echo "Three-Word Dictionary Analysis Suite"
echo "====================================="
echo ""

scripts=(
    "word_length_distribution.py"
    "visualization_histogram.py" 
    "character_analysis.py"
    "security_comparison.py"
    "comprehensive_summary.py"
)

for script in "${scripts[@]}"; do
    echo "======================================"
    echo "Running $script"
    echo "======================================"
    python3 "$script"
    echo ""
    echo ""
    
    # Add a pause between scripts to make output readable
    if [[ "$1" != "--no-pause" ]]; then
        read -p "Press Enter to continue to next analysis..."
        echo ""
    fi
done

echo "======================================"
echo "Analysis complete!"
echo "======================================"
echo ""
echo "For more details, see:"
echo "• ../3_word_analysis.md - Complete analysis report"
echo "• README.md - Script documentation"
echo "• Individual .py files - Source code and comments"