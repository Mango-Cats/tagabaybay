import morfessor

class MorphologyParser:
    def __init__(self, model_path):
        self.model = morfessor.MorfessorIO().read_binary_model_file(model_path)

    def parse(self, word):
        segments = self.model.viterbi_segment(word)[0]
        return segments
    
if __name__ == "__main__":
    parser = MorphologyParser("path/to/your/model.bin")
    word = "unhappiness"
    segments = parser.parse(word)
    print(f"Segments for '{word}': {segments}")