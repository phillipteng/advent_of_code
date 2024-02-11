import collections

class Solution:
    def get_answer(self, matrix):
        answer = 0
        convert_hand_to_bid = {}

        # Five of a kind, Full house, etc
        hand_buckets = collections.defaultdict(list)
        for row in matrix:
            hand = row[0]
            bid = row[1]
            convert_hand_to_bid[hand] = bid
            card_freqs = collections.Counter(hand)

            # full house is a special case of 3 of a kind + pair
            max_freq = 0
            number_of_pairs = 0
            for card, freq in card_freqs.items():
                max_freq = max(max_freq, freq)
                if freq == 2:
                    number_of_pairs += 1

            # special case is full house otherwise just go by max-freq to sort
            if max_freq == 3 and number_of_pairs == 1:
                hand_buckets["Full house"].append(hand)
            elif number_of_pairs == 2:
                hand_buckets["Two pair"].append(hand)
            elif number_of_pairs == 1:
                hand_buckets["One pair"].append(hand)
            elif max_freq == 1:
                hand_buckets["High card"].append(hand)
            elif max_freq == 3:
                hand_buckets["Three of a kind"].append(hand)
            elif max_freq == 4:
                hand_buckets["Four of a kind"].append(hand)
            elif max_freq == 5:
                hand_buckets["Five of a kind"].append(hand)

        # go through from the buckets of the hands and sort by strength within
        rank = 1
        hand_strengths = ["High card", "One pair", "Two pair", "Three of a kind", "Full house",
                          "Four of a kind", "Five of a kind"]

        def poker_card_compare(card):
            rank_values = {"2": 2, "3": 3, "4": 4, "5": 5, "6": 6, "7": 7, "8": 8, "9": 9, "T": 10, "J": 11,
                           "Q": 12, "K": 13, "A": 14}
            ordering = []
            for letter in card:
                ordering.append(rank_values[letter])
            return ordering

        for hand_strength in hand_strengths:
            if hand_buckets[hand_strength]:
                hand_buckets[hand_strength].sort(key = poker_card_compare)
                for hand in hand_buckets[hand_strength]:
                    answer += rank * int(convert_hand_to_bid[hand])
                    rank += 1

        return answer







def main():
    def read_file_to_matrix(filename):
        matrix = []  # Create an empty list to store the matrix
        with open(filename, 'r') as file:
            for line in file:
                row = line.strip().split()  # Strip whitespace and split by whitespace
                matrix.append(row)  # Add the row to the matrix
        return matrix

    # Example usage
    matrix = read_file_to_matrix("inputs.txt")
    print(Solution().get_answer(matrix))

if __name__ == "__main__":
    main()

