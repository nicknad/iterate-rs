## Core Algorithms

### 1 The "Stale Note" Algorithm (Enforced Learning)

The system identifies notes requiring summarization based on:

Age: (CurrentDate - EntryDate) > ConfiguredThreshold (Default: 2 weeks).
Status: IsSummarized == FALSE.
Batching: If Count(StaleNotes) > BatchSize, trigger "Reflection Mode."
