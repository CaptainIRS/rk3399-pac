#[doc = "Register `PCIE_VF_BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S` reader"]
pub type R = crate::R<PcieVfBistHeaderTypeLatencyTimerAndCacheLineSizeSSpec>;
#[doc = "Field `CLS` reader - Cache Line Size \\[CLS\\]\n\nReserved"]
pub type ClsR = crate::FieldReader;
#[doc = "Field `LT` reader - Latency Timer \\[LT\\]\n\nReserved"]
pub type LtR = crate::FieldReader;
#[doc = "Field `HT` reader - Header Type \\[HT\\]\n\nReserved"]
pub type HtR = crate::FieldReader;
#[doc = "Field `DT` reader - Device Type \\[DT\\]\n\nIdentifies whether the device\n\nsupports a single Function or\n\nmultiple Functions. This bit is read as\n\n0 when only Function 0 has been\n\nenabled in the Physical Function\n\nConfiguration Register (in the local\n\nmanagement block). Reserved for\n\nVFs"]
pub type DtR = crate::BitReader;
#[doc = "Field `BR` reader - BIST Register \\[BR\\]\n\nReserved"]
pub type BrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Cache Line Size \\[CLS\\]\n\nReserved"]
    #[inline(always)]
    pub fn cls(&self) -> ClsR {
        ClsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Latency Timer \\[LT\\]\n\nReserved"]
    #[inline(always)]
    pub fn lt(&self) -> LtR {
        LtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - Header Type \\[HT\\]\n\nReserved"]
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Device Type \\[DT\\]\n\nIdentifies whether the device\n\nsupports a single Function or\n\nmultiple Functions. This bit is read as\n\n0 when only Function 0 has been\n\nenabled in the Physical Function\n\nConfiguration Register (in the local\n\nmanagement block). Reserved for\n\nVFs"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - BIST Register \\[BR\\]\n\nReserved"]
    #[inline(always)]
    pub fn br(&self) -> BrR {
        BrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "BIST, Header Type, Latency Timer and Cache Line Size Registers\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_bist_header_type_latency_timer_and_cache_line_size_s::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfBistHeaderTypeLatencyTimerAndCacheLineSizeSSpec;
impl crate::RegisterSpec for PcieVfBistHeaderTypeLatencyTimerAndCacheLineSizeSSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_bist_header_type_latency_timer_and_cache_line_size_s::R`](R) reader structure"]
impl crate::Readable for PcieVfBistHeaderTypeLatencyTimerAndCacheLineSizeSSpec {}
#[doc = "`reset()` method sets PCIE_VF_BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S to value 0"]
impl crate::Resettable for PcieVfBistHeaderTypeLatencyTimerAndCacheLineSizeSSpec {
    const RESET_VALUE: u32 = 0;
}
