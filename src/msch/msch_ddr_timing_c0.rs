#[doc = "Register `MSCH_DdrTimingC0` reader"]
pub type R = crate::R<MschDdrTimingC0Spec>;
#[doc = "Register `MSCH_DdrTimingC0` writer"]
pub type W = crate::W<MschDdrTimingC0Spec>;
#[doc = "Field `BURSTPENALTY` reader - DRAM burst duration on the DRAM data bus in scheduler clock\n\ncycles.\n\nThe field must be set to Nd /Ns, where:\n\nNd is the number of DRAM cycles needed to process a DRAM burst\n\nof determined size, expressed in bytes.\n\nNs is the minimum number of scheduler cycles to process a DRAM\n\nburst of the same size."]
pub type BurstpenaltyR = crate::FieldReader;
#[doc = "Field `BURSTPENALTY` writer - DRAM burst duration on the DRAM data bus in scheduler clock\n\ncycles.\n\nThe field must be set to Nd /Ns, where:\n\nNd is the number of DRAM cycles needed to process a DRAM burst\n\nof determined size, expressed in bytes.\n\nNs is the minimum number of scheduler cycles to process a DRAM\n\nburst of the same size."]
pub type BurstpenaltyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRTOMWR` reader - Number of scheduler clock cycles between the last write data to the\n\nfirst data of a masked write command on the same bank.\n\nThis field must be set to 3xBurstPenalty, and must be set to zero for\n\nthe other DRAM."]
pub type WrtomwrR = crate::FieldReader;
#[doc = "Field `WRTOMWR` writer - Number of scheduler clock cycles between the last write data to the\n\nfirst data of a masked write command on the same bank.\n\nThis field must be set to 3xBurstPenalty, and must be set to zero for\n\nthe other DRAM."]
pub type WrtomwrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - DRAM burst duration on the DRAM data bus in scheduler clock\n\ncycles.\n\nThe field must be set to Nd /Ns, where:\n\nNd is the number of DRAM cycles needed to process a DRAM burst\n\nof determined size, expressed in bytes.\n\nNs is the minimum number of scheduler cycles to process a DRAM\n\nburst of the same size."]
    #[inline(always)]
    pub fn burstpenalty(&self) -> BurstpenaltyR {
        BurstpenaltyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Number of scheduler clock cycles between the last write data to the\n\nfirst data of a masked write command on the same bank.\n\nThis field must be set to 3xBurstPenalty, and must be set to zero for\n\nthe other DRAM."]
    #[inline(always)]
    pub fn wrtomwr(&self) -> WrtomwrR {
        WrtomwrR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRAM burst duration on the DRAM data bus in scheduler clock\n\ncycles.\n\nThe field must be set to Nd /Ns, where:\n\nNd is the number of DRAM cycles needed to process a DRAM burst\n\nof determined size, expressed in bytes.\n\nNs is the minimum number of scheduler cycles to process a DRAM\n\nburst of the same size."]
    #[inline(always)]
    #[must_use]
    pub fn burstpenalty(&mut self) -> BurstpenaltyW<MschDdrTimingC0Spec> {
        BurstpenaltyW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Number of scheduler clock cycles between the last write data to the\n\nfirst data of a masked write command on the same bank.\n\nThis field must be set to 3xBurstPenalty, and must be set to zero for\n\nthe other DRAM."]
    #[inline(always)]
    #[must_use]
    pub fn wrtomwr(&mut self) -> WrtomwrW<MschDdrTimingC0Spec> {
        WrtomwrW::new(self, 8)
    }
}
#[doc = "DdrTimingC bank 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_ddr_timing_c0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_ddr_timing_c0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MschDdrTimingC0Spec;
impl crate::RegisterSpec for MschDdrTimingC0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msch_ddr_timing_c0::R`](R) reader structure"]
impl crate::Readable for MschDdrTimingC0Spec {}
#[doc = "`write(|w| ..)` method takes [`msch_ddr_timing_c0::W`](W) writer structure"]
impl crate::Writable for MschDdrTimingC0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSCH_DdrTimingC0 to value 0x02"]
impl crate::Resettable for MschDdrTimingC0Spec {
    const RESET_VALUE: u32 = 0x02;
}
