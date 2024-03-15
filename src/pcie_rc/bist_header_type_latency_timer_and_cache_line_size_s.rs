#[doc = "Register `BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S` reader"]
pub type R = crate::R<BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec>;
#[doc = "Register `BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S` writer"]
pub type W = crate::W<BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec>;
#[doc = "Field `CLS` reader - Cache Line Size \\[CLS\\]
Cache Line Size Register defined in PCI Specifications 3.0. This field can be read or written, both from the link and from the local management bus, but its value is not used."]
pub type ClsR = crate::FieldReader;
#[doc = "Field `CLS` writer - Cache Line Size \\[CLS\\]
Cache Line Size Register defined in PCI Specifications 3.0. This field can be read or written, both from the link and from the local management bus, but its value is not used."]
pub type ClsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LT` reader - Latency Timer \\[LT\\]
This is an unused field and is hardwired to 0."]
pub type LtR = crate::FieldReader;
#[doc = "Field `HT` reader - Header Type \\[HT\\]
Identifies format of header. This field is hardwired to 1."]
pub type HtR = crate::FieldReader;
#[doc = "Field `DT` reader - Device Type \\[DT\\]
Identifies whether the device supports a single Function or multiple Functions. Hardwired to zero"]
pub type DtR = crate::BitReader;
#[doc = "Field `BR` reader - BIST Register \\[BR\\]
BIST control register. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type BrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Cache Line Size \\[CLS\\]
Cache Line Size Register defined in PCI Specifications 3.0. This field can be read or written, both from the link and from the local management bus, but its value is not used."]
    #[inline(always)]
    pub fn cls(&self) -> ClsR {
        ClsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Latency Timer \\[LT\\]
This is an unused field and is hardwired to 0."]
    #[inline(always)]
    pub fn lt(&self) -> LtR {
        LtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - Header Type \\[HT\\]
Identifies format of header. This field is hardwired to 1."]
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Device Type \\[DT\\]
Identifies whether the device supports a single Function or multiple Functions. Hardwired to zero"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - BIST Register \\[BR\\]
BIST control register. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn br(&self) -> BrR {
        BrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cache Line Size \\[CLS\\]
Cache Line Size Register defined in PCI Specifications 3.0. This field can be read or written, both from the link and from the local management bus, but its value is not used."]
    #[inline(always)]
    #[must_use]
    pub fn cls(&mut self) -> ClsW<BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec> {
        ClsW::new(self, 0)
    }
}
#[doc = "BIST, Header Type, Latency Timer and Cache Line Size Registers BIST control register. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bist_header_type_latency_timer_and_cache_line_size_s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bist_header_type_latency_timer_and_cache_line_size_s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec;
impl crate::RegisterSpec for BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bist_header_type_latency_timer_and_cache_line_size_s::R`](R) reader structure"]
impl crate::Readable for BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec {}
#[doc = "`write(|w| ..)` method takes [`bist_header_type_latency_timer_and_cache_line_size_s::W`](W) writer structure"]
impl crate::Writable for BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIST_HEADER_TYPE_LATENCY_TIMER_AND_CACHE_LINE_SIZE_S to value 0x0001_0000"]
impl crate::Resettable for BistHeaderTypeLatencyTimerAndCacheLineSizeSSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
