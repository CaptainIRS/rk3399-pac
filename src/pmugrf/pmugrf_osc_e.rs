#[doc = "Register `PMUGRF_OSC_E` reader"]
pub type R = crate::R<PmugrfOscESpec>;
#[doc = "Register `PMUGRF_OSC_E` writer"]
pub type W = crate::W<PmugrfOscESpec>;
#[doc = "Field `OSC_E` reader - 24M OSC drive strenth"]
pub type OscER = crate::FieldReader;
#[doc = "Field `OSC_E` writer - 24M OSC drive strenth"]
pub type OscEW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRITE_ENABLE` reader - When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; When bit 18=1, bit 2 can be written by software . When bit 18=0, bit 2 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader;
#[doc = "Field `WRITE_ENABLE` writer - When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; When bit 18=1, bit 2 can be written by software . When bit 18=0, bit 2 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 24M OSC drive strenth"]
    #[inline(always)]
    pub fn osc_e(&self) -> OscER {
        OscER::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; When bit 18=1, bit 2 can be written by software . When bit 18=0, bit 2 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 24M OSC drive strenth"]
    #[inline(always)]
    #[must_use]
    pub fn osc_e(&mut self) -> OscEW<PmugrfOscESpec> {
        OscEW::new(self, 0)
    }
    #[doc = "Bits 16:18 - When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; When bit 18=1, bit 2 can be written by software . When bit 18=0, bit 2 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfOscESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "OSC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_osc_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_osc_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfOscESpec;
impl crate::RegisterSpec for PmugrfOscESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_osc_e::R`](R) reader structure"]
impl crate::Readable for PmugrfOscESpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_osc_e::W`](W) writer structure"]
impl crate::Writable for PmugrfOscESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_OSC_E to value 0x06"]
impl crate::Resettable for PmugrfOscESpec {
    const RESET_VALUE: u32 = 0x06;
}
