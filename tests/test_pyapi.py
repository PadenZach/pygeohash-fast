from pygeohash_fast import encode, decode, decode_many, encode_many

class TestClass:
    def test_encode_works(self):
        assert encode(-72.747917, 45.207615, 5) == "f2h30"
    
    def test_decode_works(self):
        assert decode("f2h30") == (-72.75146484375, 45.19775390625, 0.02197265625, 0.02197265625)
    
    def test_decode_many_works(self):
        assert decode_many(["f2h30", "f2h30"]) == [(-72.75146484375, 45.19775390625), (-72.75146484375, 45.19775390625)]

    def test_encode_many_works(self):
        lats = [47.1, 35.204]
        lngs = [-76.6, -80.8501]
        expected = ["f23e", "dnq8"]
        assert encode_many(lngs, lats, 4) == expected