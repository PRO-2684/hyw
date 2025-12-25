//! Data and basic structures for hyw.

use std::fmt;

// Constructed by using https://github.com/PRO-2684/pinchar and further filtering from https://hanzicraft.com/lists/frequency.

/// Characters with pinyin "he".
pub static HE: &[char] = &['何', '劾', '合', '呵', '和', '喝', '嗬', '壑', '曷', '核', '河', '涸', '盇', '盉', '盍', '盒', '礉', '禾', '粭', '翮', '翯', '荷', '菏', '蚵', '蠚', '袔', '褐', '詥', '诃', '贺', '赫', '郃', '阂', '阖', '鞨', '颌', '饸', '魺', '鲄', '鶡', '鸖', '鹖', '鹤', '麧', '龁', '龢'];

/// Characters with pinyin "yi".
pub static YI: &[char] = &['一', '义', '乙', '亦', '亿', '以', '仪', '伊', '佚', '佾', '依', '倚', '刈', '劓', '医', '呓', '咦', '咿', '噫', '圯', '埸', '壹', '夷', '奕', '姨', '宜', '屹', '峄', '嶷', '已', '异', '弈', '弋', '彝', '役', '忆', '怡', '怿', '悒', '意', '懿', '抑', '挹', '揖', '旖', '易', '椅', '欹', '殪', '毅', '沂', '溢', '漪', '熠', '猗', '疑', '疫', '痍', '瘗', '癔', '益', '眙', '睪', '矣', '礒', '祎', '移', '笖', '箷', '簃', '绎', '缢', '羠', '羿', '翊', '翌', '翳', '翼', '耴', '肄', '肊', '胰', '臆', '舣', '艺', '苅', '苡', '蓺', '薏', '藙', '蚁', '蛜', '蛡', '蜴', '螘', '螠', '衣', '袘', '袣', '裔', '裛', '褹', '訑', '讉', '讛', '议', '译', '诒', '诣', '谊', '豷', '貤', '賹', '贀', '贻', '踦', '轶', '迆', '迤', '迻', '逸', '遗', '邑', '钇', '铱', '镒', '镱', '頉', '顗', '颐', '饐', '饴', '驿', '鮨', '鶃', '鶍', '鷁', '鷖', '鷧', '黟', '齮', '齸'];

/// Characters with pinyin "wei".
pub static WEI: &[char] = &['为', '伟', '伪', '位', '偎', '卫', '危', '味', '唯', '喂', '围', '圩', '委', '威', '娓', '尉', '尾', '嵬', '巍', '帏', '帷', '微', '惟', '慰', '未', '桅', '沩', '洧', '涠', '渭', '潍', '炜', '煨', '猥', '猬', '玮', '畏', '痿', '硊', '硙', '磈', '纬', '维', '胃', '艉', '苇', '苿', '萎', '葨', '葳', '蔚', '蔿', '薇', '薳', '藯', '蜼', '讏', '诿', '谓', '躛', '軎', '违', '逶', '鄬', '鍏', '鏏', '闱', '隈', '霨', '韦', '韪', '頠', '骩', '骫', '魏', '鮇', '鮠', '鲔', '鳂', '鳚'];

/// A combination of these characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hyw {
    /// Index of character with pinyin "he".
    he: usize,
    /// Index of character with pinyin "yi".
    yi: usize,
    /// Index of character with pinyin "wei".
    wei: usize,
}

impl Hyw {
    /// Create a new [`Hyw`].
    pub const fn new() -> Self {
        Self { he: 0, yi: 0, wei: 0 }
    }

    /// Create a [`Hyw`] from combined index. If the index is out of bounds, return `None`.
    pub const fn from_index(index: usize) -> Option<Self> {
        let he_len = HE.len();
        let yi_len = YI.len();
        let wei_len = WEI.len();
        let total_len = he_len * yi_len * wei_len;

        if index >= total_len {
            return None;
        }

        let he = index / (yi_len * wei_len);
        let yi = (index / wei_len) % yi_len;
        let wei = index % wei_len;

        Some(Self { he, yi, wei })
    }

    /// Get the combined index of this [`Hyw`].
    pub const fn to_index(&self) -> usize {
        let yi_len = YI.len();
        let wei_len = WEI.len();
        self.he * yi_len * wei_len + self.yi * wei_len + self.wei
    }

    /// Get the next [`Hyw`] combination. If it is the last combination, return `None`.
    pub const fn next(&self) -> Option<Self> {
        let mut he = self.he;
        let mut yi = self.yi;
        let mut wei = self.wei + 1;

        if wei >= WEI.len() {
            wei = 0;
            yi += 1;
            if yi >= YI.len() {
                yi = 0;
                he += 1;
                if he >= HE.len() {
                    return None;
                }
            }
        }

        Some(Self { he, yi, wei })
    }

    /// Get the previous [`Hyw`] combination. If it is the first combination, return `None`.
    pub const fn previous(&self) -> Option<Self> {
        let mut he = self.he;
        let mut yi = self.yi;
        let mut wei = self.wei;

        if wei == 0 {
            wei = WEI.len() - 1;
            if yi == 0 {
                yi = YI.len() - 1;
                if he == 0 {
                    return None;
                } else {
                    he -= 1;
                }
            } else {
                yi -= 1;
            }
        } else {
            wei -= 1;
        }

        Some(Self { he, yi, wei })
    }

    /// Returns an iterator over all Hyw combinations starting from this one
    pub const fn iter_from(self) -> HywIterator {
        HywIterator { current: Some(self) }
    }

    /// Returns an iterator over all possible Hyw combinations
    pub const fn all() -> HywIterator {
        HywIterator { current: Some(Hyw::new()) }
    }

    /// Set the indice of character with pinyin "he". If the index is out of bounds, return `false` and do not modify the value.
    pub const fn set_he(&mut self, he: usize) -> bool {
        if he < HE.len() {
            self.he = he;
            true
        } else {
            false
        }
    }

    /// Set the indice of character with pinyin "yi". If the index is out of bounds, return `false` and do not modify the value.
    pub const fn set_yi(&mut self, yi: usize) -> bool {
        if yi < YI.len() {
            self.yi = yi;
            true
        } else {
            false
        }
    }

    /// Set the indice of character with pinyin "wei". If the index is out of bounds, return `false` and do not modify the value.
    pub const fn set_wei(&mut self, wei: usize) -> bool {
        if wei < WEI.len() {
            self.wei = wei;
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Hyw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", HE[self.he], YI[self.yi], WEI[self.wei])
    }
}

impl Default for Hyw {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over all possible [`Hyw`] combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HywIterator {
    current: Option<Hyw>,
}

impl Iterator for HywIterator {
    type Item = Hyw;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = current.next();
        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for HywIterator {
    fn len(&self) -> usize {
        match self.current {
            Some(hyw) => HE.len() * YI.len() * WEI.len() - hyw.to_index(),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyw_new() {
        let hyw = Hyw::new();
        assert_eq!(hyw.to_index(), 0);
        assert_eq!(hyw.to_string(), format!("{}{}{}", HE[0], YI[0], WEI[0]));
    }

    #[test]
    fn test_hyw_from_index() {
        let hyw = Hyw::from_index(0).unwrap();
        assert_eq!(hyw.to_string(), format!("{}{}{}", HE[0], YI[0], WEI[0]));

        let hyw = Hyw::from_index(1).unwrap();
        assert_eq!(hyw.to_string(), format!("{}{}{}", HE[0], YI[0], WEI[1]));

        // Test boundary
        let total = HE.len() * YI.len() * WEI.len();
        assert!(Hyw::from_index(total - 1).is_some());
        assert!(Hyw::from_index(total).is_none());
    }

    #[test]
    fn test_hyw_to_index_roundtrip() {
        for i in [0, 1, 100, 1000, 10000] {
            if let Some(hyw) = Hyw::from_index(i) {
                assert_eq!(hyw.to_index(), i);
            }
        }
    }

    #[test]
    fn test_hyw_next() {
        let mut hyw = Hyw::new();
        let next = hyw.next().unwrap();
        assert_eq!(next.to_index(), 1);

        // Test wraparound
        let last_wei = WEI.len() - 1;
        hyw.set_wei(last_wei);
        let next = hyw.next().unwrap();
        assert_eq!(next.to_index(), hyw.to_index() + 1);

        // Test last combination
        let total = HE.len() * YI.len() * WEI.len();
        let last = Hyw::from_index(total - 1).unwrap();
        assert!(last.next().is_none());
    }

    #[test]
    fn test_hyw_previous() {
        let hyw = Hyw::from_index(100).unwrap();
        let prev = hyw.previous().unwrap();
        assert_eq!(prev.to_index(), 99);

        // Test first combination
        let first = Hyw::new();
        assert!(first.previous().is_none());

        // Test wraparound
        let mut hyw = Hyw::new();
        hyw.set_yi(1);
        let prev = hyw.previous().unwrap();
        assert_eq!(prev.to_index(), hyw.to_index() - 1);
    }

    #[test]
    fn test_iterator_basic() {
        let mut iter = Hyw::all();
        let first = iter.next().unwrap();
        assert_eq!(first, Hyw::new());

        let second = iter.next().unwrap();
        assert_eq!(second, Hyw::new().next().unwrap());
    }

    #[test]
    fn test_iterator_count() {
        let total = HE.len() * YI.len() * WEI.len();
        let count = Hyw::all().count();
        assert_eq!(count, total);
    }

    #[test]
    fn test_iterator_take() {
        let first_10: Vec<_> = Hyw::all().take(10).collect();
        assert_eq!(first_10.len(), 10);
        assert_eq!(first_10[0], Hyw::new());
        assert_eq!(first_10[9], Hyw::from_index(9).unwrap());
    }

    #[test]
    fn test_iterator_skip() {
        let start_from_100 = Hyw::all().skip(100).next().unwrap();
        assert_eq!(start_from_100, Hyw::from_index(100).unwrap());
    }

    #[test]
    fn test_iterator_iter_from() {
        let start = Hyw::from_index(50).unwrap();
        let mut iter = start.iter_from();
        assert_eq!(iter.next().unwrap(), start);
        assert_eq!(iter.next().unwrap(), start.next().unwrap());
    }

    #[test]
    fn test_iterator_size_hint() {
        let total = HE.len() * YI.len() * WEI.len();
        let iter = Hyw::all();
        assert_eq!(iter.size_hint(), (total, Some(total)));

        let iter = Hyw::all().skip(100);
        let (lower, upper) = iter.size_hint();
        assert_eq!(lower, total - 100);
        assert_eq!(upper, Some(total - 100));
    }

    #[test]
    fn test_iterator_len() {
        let total = HE.len() * YI.len() * WEI.len();
        let iter = Hyw::all();
        assert_eq!(iter.len(), total);

        let iter = Hyw::from_index(100).unwrap().iter_from();
        assert_eq!(iter.len(), total - 100);

        // Empty iterator
        let last = Hyw::from_index(total - 1).unwrap();
        let mut iter = last.iter_from();
        iter.next(); // consume the last item
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn test_iterator_exhausted() {
        let last = Hyw::from_index(HE.len() * YI.len() * WEI.len() - 1).unwrap();
        let mut iter = last.iter_from();
        assert_eq!(iter.next(), Some(last));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None); // Still None
    }

    #[test]
    fn test_set_methods() {
        let mut hyw = Hyw::new();

        assert!(hyw.set_he(10));
        assert!(hyw.set_yi(20));
        assert!(hyw.set_wei(30));

        assert_eq!(hyw.to_string(), format!("{}{}{}", HE[10], YI[20], WEI[30]));

        // Out of bounds
        assert!(!hyw.set_he(HE.len()));
        assert!(!hyw.set_yi(YI.len()));
        assert!(!hyw.set_wei(WEI.len()));
    }

    #[test]
    fn test_display_format() {
        let hyw = Hyw::new();
        let display = format!("{}", hyw);
        let expected = format!("{}{}{}", HE[0], YI[0], WEI[0]);
        assert_eq!(display, expected);
    }
}
