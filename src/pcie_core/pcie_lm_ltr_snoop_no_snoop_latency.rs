#[doc = "Register `PCIE_LM_LTR_SNOOP_NO_SNOOP_LATENCY` reader"]
pub type R = crate::R<PcieLmLtrSnoopNoSnoopLatencySpec>;
#[doc = "Register `PCIE_LM_LTR_SNOOP_NO_SNOOP_LATENCY` writer"]
pub type W = crate::W<PcieLmLtrSnoopNoSnoopLatencySpec>;
#[doc = "Field `NSLV` reader - No-Snoop Latency Value \\[NSLV\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe No-Snoop Latency Value field of\n\nthe LTR message."]
pub type NslvR = crate::FieldReader<u16>;
#[doc = "Field `NSLV` writer - No-Snoop Latency Value \\[NSLV\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe No-Snoop Latency Value field of\n\nthe LTR message."]
pub type NslvW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NSLS` reader - No-Snoop Latency Scale \\[NSLS\\]\n\nThe client software must program\n\nthis field with the value to be sent\n\nin the No-Snoop Latency Scale field\n\nof the LTR message."]
pub type NslsR = crate::FieldReader;
#[doc = "Field `NSLS` writer - No-Snoop Latency Scale \\[NSLS\\]\n\nThe client software must program\n\nthis field with the value to be sent\n\nin the No-Snoop Latency Scale field\n\nof the LTR message."]
pub type NslsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R12` reader - Reserved \\[R12\\]\n\n(no description)"]
pub type R12R = crate::FieldReader;
#[doc = "Field `NSLR` reader - No-Snoop Latency Requirement \\[NSLR\\]\n\nThe client software must set this bit\n\nto 1 to set the No-Snoop Latency\n\nRequirement bit in the LTR message\n\nto be sent."]
pub type NslrR = crate::BitReader;
#[doc = "Field `NSLR` writer - No-Snoop Latency Requirement \\[NSLR\\]\n\nThe client software must set this bit\n\nto 1 to set the No-Snoop Latency\n\nRequirement bit in the LTR message\n\nto be sent."]
pub type NslrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV` reader - Snoop Latency Value \\[SLV\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe Snoop Latency Value field of the\n\nLTR message."]
pub type SlvR = crate::FieldReader<u16>;
#[doc = "Field `SLV` writer - Snoop Latency Value \\[SLV\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe Snoop Latency Value field of the\n\nLTR message."]
pub type SlvW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SLS` reader - Snoop Latency Scale \\[SLS\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe Snoop Latency Scale field of the\n\nLTR message."]
pub type SlsR = crate::FieldReader;
#[doc = "Field `SLS` writer - Snoop Latency Scale \\[SLS\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe Snoop Latency Scale field of the\n\nLTR message."]
pub type SlsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `R13` reader - Reserved \\[R13\\]\n\nReserved"]
pub type R13R = crate::FieldReader;
#[doc = "Field `SL` reader - Snoop Latency \\[SL\\]\n\nThe client software must set this bit\n\nto 1 to set the Snoop Latency\n\nRequirement bit in the LTR message\n\nto be sent."]
pub type SlR = crate::BitReader;
#[doc = "Field `SL` writer - Snoop Latency \\[SL\\]\n\nThe client software must set this bit\n\nto 1 to set the Snoop Latency\n\nRequirement bit in the LTR message\n\nto be sent."]
pub type SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - No-Snoop Latency Value \\[NSLV\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe No-Snoop Latency Value field of\n\nthe LTR message."]
    #[inline(always)]
    pub fn nslv(&self) -> NslvR {
        NslvR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:12 - No-Snoop Latency Scale \\[NSLS\\]\n\nThe client software must program\n\nthis field with the value to be sent\n\nin the No-Snoop Latency Scale field\n\nof the LTR message."]
    #[inline(always)]
    pub fn nsls(&self) -> NslsR {
        NslsR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:14 - Reserved \\[R12\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - No-Snoop Latency Requirement \\[NSLR\\]\n\nThe client software must set this bit\n\nto 1 to set the No-Snoop Latency\n\nRequirement bit in the LTR message\n\nto be sent."]
    #[inline(always)]
    pub fn nslr(&self) -> NslrR {
        NslrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Snoop Latency Value \\[SLV\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe Snoop Latency Value field of the\n\nLTR message."]
    #[inline(always)]
    pub fn slv(&self) -> SlvR {
        SlvR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:28 - Snoop Latency Scale \\[SLS\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe Snoop Latency Scale field of the\n\nLTR message."]
    #[inline(always)]
    pub fn sls(&self) -> SlsR {
        SlsR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:30 - Reserved \\[R13\\]\n\nReserved"]
    #[inline(always)]
    pub fn r13(&self) -> R13R {
        R13R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Snoop Latency \\[SL\\]\n\nThe client software must set this bit\n\nto 1 to set the Snoop Latency\n\nRequirement bit in the LTR message\n\nto be sent."]
    #[inline(always)]
    pub fn sl(&self) -> SlR {
        SlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - No-Snoop Latency Value \\[NSLV\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe No-Snoop Latency Value field of\n\nthe LTR message."]
    #[inline(always)]
    #[must_use]
    pub fn nslv(&mut self) -> NslvW<PcieLmLtrSnoopNoSnoopLatencySpec> {
        NslvW::new(self, 0)
    }
    #[doc = "Bits 10:12 - No-Snoop Latency Scale \\[NSLS\\]\n\nThe client software must program\n\nthis field with the value to be sent\n\nin the No-Snoop Latency Scale field\n\nof the LTR message."]
    #[inline(always)]
    #[must_use]
    pub fn nsls(&mut self) -> NslsW<PcieLmLtrSnoopNoSnoopLatencySpec> {
        NslsW::new(self, 10)
    }
    #[doc = "Bit 15 - No-Snoop Latency Requirement \\[NSLR\\]\n\nThe client software must set this bit\n\nto 1 to set the No-Snoop Latency\n\nRequirement bit in the LTR message\n\nto be sent."]
    #[inline(always)]
    #[must_use]
    pub fn nslr(&mut self) -> NslrW<PcieLmLtrSnoopNoSnoopLatencySpec> {
        NslrW::new(self, 15)
    }
    #[doc = "Bits 16:25 - Snoop Latency Value \\[SLV\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe Snoop Latency Value field of the\n\nLTR message."]
    #[inline(always)]
    #[must_use]
    pub fn slv(&mut self) -> SlvW<PcieLmLtrSnoopNoSnoopLatencySpec> {
        SlvW::new(self, 16)
    }
    #[doc = "Bits 26:28 - Snoop Latency Scale \\[SLS\\]\n\nThe client software must program\n\nthis field with the value to be sent in\n\nthe Snoop Latency Scale field of the\n\nLTR message."]
    #[inline(always)]
    #[must_use]
    pub fn sls(&mut self) -> SlsW<PcieLmLtrSnoopNoSnoopLatencySpec> {
        SlsW::new(self, 26)
    }
    #[doc = "Bit 31 - Snoop Latency \\[SL\\]\n\nThe client software must set this bit\n\nto 1 to set the Snoop Latency\n\nRequirement bit in the LTR message\n\nto be sent."]
    #[inline(always)]
    #[must_use]
    pub fn sl(&mut self) -> SlW<PcieLmLtrSnoopNoSnoopLatencySpec> {
        SlW::new(self, 31)
    }
}
#[doc = "LTR Snoop/No-Snoop Latency Register\n\nThe client software must set this bit\n\nto 1 to set the Snoop Latency\n\nRequirement bit in the LTR message\n\nto be sent.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_ltr_snoop_no_snoop_latency::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_ltr_snoop_no_snoop_latency::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmLtrSnoopNoSnoopLatencySpec;
impl crate::RegisterSpec for PcieLmLtrSnoopNoSnoopLatencySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_ltr_snoop_no_snoop_latency::R`](R) reader structure"]
impl crate::Readable for PcieLmLtrSnoopNoSnoopLatencySpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_ltr_snoop_no_snoop_latency::W`](W) writer structure"]
impl crate::Writable for PcieLmLtrSnoopNoSnoopLatencySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_LTR_SNOOP_NO_SNOOP_LATENCY to value 0"]
impl crate::Resettable for PcieLmLtrSnoopNoSnoopLatencySpec {
    const RESET_VALUE: u32 = 0;
}
