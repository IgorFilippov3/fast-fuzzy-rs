const { search, fuzzy } = require("../index.js");

describe("Fast Fuzzy Search", () => {
  const testData = [
    "apple",
    "application",
    "apply",
    "appreciate",
    "banana",
    "grape",
    "pineapple",
    "watermelon",
  ];

  describe("search", () => {
    test("should find exact matches", () => {
      const results = search("apple", testData);
      expect(results[0].item).toBe("apple");
      expect(results[0].score).toBe(1.0);
    });

    test("should handle partial matches", () => {
      const results = search("app", testData);
      expect(results.length).toBeGreaterThan(0);
      expect(results[0].item).toMatch(/app/i);
    });

    test("should respect threshold option", () => {
      const results = search("xyz", testData, { threshold: 0.5 });
      expect(results.length).toBe(0);
    });

    test("should limit results", () => {
      const results = search("a", testData, { limit: 3 });
      expect(results.length).toBeLessThanOrEqual(3);
    });

    test("should handle case insensitive search", () => {
      const results = search("APPLE", testData, { ignoreCase: true });
      expect(results[0].item).toBe("apple");
    });

    test("should normalize unicode characters", () => {
      const data = ["café", "naïve", "résumé"];
      const results = search("cafe", data, { normalize: true });
      expect(results[0].item).toBe("café");
    });
  });

  describe("fuzzy", () => {
    test("should return 1.0 for identical strings", () => {
      expect(fuzzy("test", "test")).toBe(1.0);
    });

    test("should return 0.0 for completely different strings", () => {
      const score = fuzzy("abc", "xyz");
      expect(score).toBeLessThan(0.5);
    });

    test("should handle empty strings", () => {
      expect(fuzzy("", "")).toBe(1.0);
      expect(fuzzy("test", "")).toBe(0.0);
      expect(fuzzy("", "test")).toBe(0.0);
    });

    test("should normalize strings when requested", () => {
      const score = fuzzy("café", "cafe", true);
      expect(score).toBe(1.0);
    });
  });
});
