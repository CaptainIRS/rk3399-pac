#[doc = "Register `BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S` reader"]
pub type R = crate::R<BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec>;
#[doc = "Field `CLS` reader - Cache Line Size \\[CLS\\]
Reserved"]
pub type ClsR = crate::FieldReader;
#[doc = "Field `LT` reader - Latency Timer \\[LT\\]
Reserved"]
pub type LtR = crate::FieldReader;
#[doc = "Field `HT` reader - Header Type \\[HT\\]
Reserved"]
pub type HtR = crate::FieldReader;
#[doc = "Field `DT` reader - Device Type \\[DT\\]
Identifies whether the device supports a single Function or multiple Functions. This bit is read as 0 when only Function 0 has been enabled in the Physical Function Configuration Register (in the local management block). Reserved for VFs"]
pub type DtR = crate::BitReader;
#[doc = "Field `BR` reader - BIST Register \\[BR\\]
Reserved"]
pub type BrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Cache Line Size \\[CLS\\]
Reserved"]
    #[inline(always)]
    pub fn cls(&self) -> ClsR {
        ClsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Latency Timer \\[LT\\]
Reserved"]
    #[inline(always)]
    pub fn lt(&self) -> LtR {
        LtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - Header Type \\[HT\\]
Reserved"]
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Device Type \\[DT\\]
Identifies whether the device supports a single Function or multiple Functions. This bit is read as 0 when only Function 0 has been enabled in the Physical Function Configuration Register (in the local management block). Reserved for VFs"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - BIST Register \\[BR\\]
Reserved"]
    #[inline(always)]
    pub fn br(&self) -> BrR {
        BrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "BIST, Header Type, Latency Timer and Cache Line Size Registers Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bist_header_type_latency_timer_and_cache_line_size_s::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec;
impl crate::RegisterSpec for BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bist_header_type_latency_timer_and_cache_line_size_s::R`](R) reader structure"]
impl crate::Readable for BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec {}
#[doc = "`reset()` method sets BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S to value 0"]
impl crate::Resettable for BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec {
    const RESET_VALUE: u32 = 0;
}
