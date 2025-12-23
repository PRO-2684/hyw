//! Data and basic structures for hyw.

use std::fmt;

/// Characters with pinyin "he".
pub static HE: &[char] = &['何', '佫', '劾', '合', '呵', '咊', '和', '哬', '啝', '喝', '嗃', '嗬', '垎', '壑', '姀', '寉', '峆', '惒', '抲', '敆', '曷', '柇', '核', '楁', '欱', '毼', '河', '涸', '渮', '澕', '焃', '煂', '熆', '熇', '燺', '爀', '狢', '癋', '皬', '盇', '盉', '盍', '盒', '碋', '礉', '禾', '秴', '穒', '篕', '籺', '粭', '紇', '翮', '翯', '荷', '菏', '萂', '蚵', '螛', '蠚', '袔', '褐', '覈', '訶', '訸', '詥', '謞', '诃', '貈', '賀', '贺', '赫', '輅', '郃', '鉌', '鑉', '闔', '阂', '阖', '靍', '靎', '靏', '鞨', '頜', '颌', '饸', '魺', '鲄', '鶡', '鶮', '鶴', '鸖', '鹖', '鹤', '麧', '齕', '龁', '龢'];

/// Characters with pinyin "yi".
pub static YI: &[char] = &['一', '乁', '乂', '义', '乊', '乙', '亄', '亦', '亿', '以', '仪', '伇', '伊', '伿', '佁', '佚', '佾', '侇', '依', '俋', '倚', '偯', '儀', '億', '兿', '冝', '刈', '劓', '劮', '勚', '勩', '匇', '匜', '医', '吚', '呓', '呭', '呹', '咦', '咿', '唈', '噫', '囈', '圛', '圯', '坄', '垼', '埶', '埸', '墿', '壱', '壹', '夁', '夷', '奕', '姨', '媐', '嫕', '嫛', '嬄', '嬑', '嬟', '宐', '宜', '宧', '寱', '寲', '屹', '峄', '峓', '崺', '嶧', '嶬', '嶷', '已', '巸', '帟', '帠', '幆', '庡', '廙', '异', '弈', '弋', '弌', '弬', '彛', '彜', '彝', '彞', '役', '忆', '怈', '怡', '怿', '恞', '悒', '悘', '悥', '意', '憶', '懌', '懿', '扅', '扆', '抑', '拸', '挹', '掜', '揖', '撎', '攺', '敡', '敼', '斁', '旑', '旖', '易', '晹', '暆', '曀', '曎', '杙', '枍', '枻', '柂', '栘', '栧', '栺', '桋', '棭', '椅', '椬', '椸', '榏', '槸', '檍', '檥', '檹', '欥', '欭', '欹', '歝', '殔', '殪', '殹', '毅', '毉', '沂', '沶', '泆', '洢', '浂', '浥', '浳', '渏', '湙', '溢', '漪', '潩', '澺', '瀷', '炈', '焲', '熠', '熤', '熪', '熼', '燚', '燡', '燱', '狋', '猗', '獈', '玴', '珆', '瑿', '瓵', '畩', '異', '疑', '疫', '痍', '痬', '瘗', '瘞', '瘱', '癔', '益', '眙', '睪', '瞖', '矣', '硛', '礒', '祎', '禕', '秇', '移', '稦', '穓', '竩', '笖', '箷', '簃', '籎', '縊', '繄', '繶', '繹', '绎', '缢', '羛', '羠', '義', '羿', '翊', '翌', '翳', '翼', '耛', '耴', '肄', '肊', '胰', '膉', '臆', '舣', '艗', '艤', '艺', '芅', '苅', '苡', '苢', '萓', '萟', '蓺', '薏', '藙', '藝', '蘙', '虉', '蚁', '蛜', '蛡', '蛦', '蜴', '螔', '螘', '螠', '蟻', '衣', '衤', '衪', '衵', '袘', '袣', '裔', '裛', '裿', '褹', '襼', '觺', '訑', '訲', '訳', '詍', '詑', '詒', '詣', '誃', '誼', '謻', '譩', '譯', '議', '讉', '讛', '议', '译', '诒', '诣', '谊', '豙', '豛', '豷', '貖', '貤', '貽', '賹', '贀', '贻', '跇', '跠', '踦', '軼', '輢', '轙', '轶', '辷', '迆', '迤', '迻', '逘', '逸', '遗', '遺', '邑', '郼', '酏', '醫', '醳', '醷', '釔', '釴', '鈘', '鈠', '鉯', '銥', '鎰', '鏔', '鐿', '钇', '铱', '镒', '镱', '陭', '隿', '霬', '靾', '頉', '頤', '頥', '顊', '顗', '颐', '飴', '饐', '饴', '駅', '驛', '驿', '骮', '鮨', '鯣', '鳦', '鶂', '鶃', '鶍', '鷁', '鷊', '鷖', '鷧', '鷾', '鸃', '鹝', '鹢', '鹥', '黓', '黟', '黳', '齮', '齸'];

/// Characters with pinyin "wei".
pub static WEI: &[char] = &['为', '亹', '伟', '伪', '位', '偉', '偎', '偽', '僞', '儰', '卫', '危', '厃', '叞', '味', '唯', '喂', '喡', '喴', '囗', '围', '圍', '圩', '墛', '壝', '委', '威', '娓', '媁', '媙', '媦', '寪', '尉', '尾', '屗', '峗', '峞', '崣', '嵔', '嵬', '嶶', '巍', '帏', '帷', '幃', '徫', '微', '惟', '愄', '愇', '慰', '懀', '捤', '揋', '揻', '撱', '斖', '暐', '未', '桅', '梶', '椲', '椳', '楲', '欈', '沩', '洈', '洧', '浘', '涠', '渨', '渭', '湋', '溈', '溦', '潍', '潙', '潿', '濰', '濻', '瀢', '炜', '為', '烓', '煀', '煒', '煟', '煨', '熭', '燰', '爲', '犚', '犩', '猥', '猬', '玮', '琟', '瑋', '璏', '畏', '痏', '痿', '癓', '硊', '硙', '碨', '磈', '磑', '維', '緭', '緯', '縅', '纬', '维', '罻', '胃', '腲', '艉', '芛', '苇', '苿', '荱', '菋', '萎', '葦', '葨', '葳', '蒍', '蓶', '蔚', '蔿', '薇', '薳', '藯', '蘶', '蜲', '蜼', '蝛', '蝟', '螱', '衛', '衞', '褽', '覣', '覹', '詴', '諉', '謂', '讆', '讏', '诿', '谓', '踓', '躗', '躛', '軎', '轊', '违', '逶', '違', '鄬', '醀', '鍏', '鍡', '鏏', '闈', '闱', '隇', '隈', '霨', '霺', '韋', '韑', '韙', '韡', '韦', '韪', '頠', '颹', '餧', '餵', '饖', '骩', '骪', '骫', '魏', '鮇', '鮠', '鮪', '鰃', '鰄', '鲔', '鳂', '鳚'];

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
