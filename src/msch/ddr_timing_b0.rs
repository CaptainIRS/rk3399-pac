#[doc = "Register `DdrTimingB0` reader"]
pub type R = crate::R<DdrTimingB0Spec>;
#[doc = "Register `DdrTimingB0` writer"]
pub type W = crate::W<DdrTimingB0Spec>;
#[doc = "Field `RDTOWR` reader - Minimum number of scheduler clock cycles between the last DRAM\n\nRead command and a Write command.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n2 ×tCkD / tCkG, for DDR2 memories.\n\n(RL –WL + 2) ×tCkD / tCKG, for DDR3 and DDR4 memories.\n\n(RL + RoundUp(tDQSCK(max) / tCKD) + tRPST - WL + tWPRE)\n\n×tCkD / tCKG , for LPDDR4 memories."]
pub type RdtowrR = crate::FieldReader;
#[doc = "Field `RDTOWR` writer - Minimum number of scheduler clock cycles between the last DRAM\n\nRead command and a Write command.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n2 ×tCkD / tCkG, for DDR2 memories.\n\n(RL –WL + 2) ×tCkD / tCKG, for DDR3 and DDR4 memories.\n\n(RL + RoundUp(tDQSCK(max) / tCKD) + tRPST - WL + tWPRE)\n\n×tCkD / tCKG , for LPDDR4 memories."]
pub type RdtowrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRTORD` reader - Minimum number of scheduler clock cycles between the last DRAM\n\nWrite command and a Read command.\n\nThe field must be set to the following value, rounded to an integer\n\nof scheduler clock cycles:\n\n(WL ×tCkD + tWTR) / tCkG, for DDR2 and DDR3 memories.\n\n(WL ×tCkD + tWTR_S) / tCkG, for DDR4 memories.\n\n((WL + 1) ×tCkD + tWTR) / tCkG, for LPDDR4 memories."]
pub type WrtordR = crate::FieldReader;
#[doc = "Field `WRTORD` writer - Minimum number of scheduler clock cycles between the last DRAM\n\nWrite command and a Read command.\n\nThe field must be set to the following value, rounded to an integer\n\nof scheduler clock cycles:\n\n(WL ×tCkD + tWTR) / tCkG, for DDR2 and DDR3 memories.\n\n(WL ×tCkD + tWTR_S) / tCkG, for DDR4 memories.\n\n((WL + 1) ×tCkD + tWTR) / tCkG, for LPDDR4 memories."]
pub type WrtordW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RRD` reader - Number of cycle between two consecutive Activate commands on\n\ndifferent Banks of the same device.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntRRD / tCkG"]
pub type RrdR = crate::FieldReader;
#[doc = "Field `RRD` writer - Number of cycle between two consecutive Activate commands on\n\ndifferent Banks of the same device.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntRRD / tCkG"]
pub type RrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FAW` reader - Field0000 Abstract\n\nNumber of cycle of the FAW period.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntFAW / tCkG"]
pub type FawR = crate::FieldReader;
#[doc = "Field `FAW` writer - Field0000 Abstract\n\nNumber of cycle of the FAW period.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntFAW / tCkG"]
pub type FawW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:4 - Minimum number of scheduler clock cycles between the last DRAM\n\nRead command and a Write command.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n2 ×tCkD / tCkG, for DDR2 memories.\n\n(RL –WL + 2) ×tCkD / tCKG, for DDR3 and DDR4 memories.\n\n(RL + RoundUp(tDQSCK(max) / tCKD) + tRPST - WL + tWPRE)\n\n×tCkD / tCKG , for LPDDR4 memories."]
    #[inline(always)]
    pub fn rdtowr(&self) -> RdtowrR {
        RdtowrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Minimum number of scheduler clock cycles between the last DRAM\n\nWrite command and a Read command.\n\nThe field must be set to the following value, rounded to an integer\n\nof scheduler clock cycles:\n\n(WL ×tCkD + tWTR) / tCkG, for DDR2 and DDR3 memories.\n\n(WL ×tCkD + tWTR_S) / tCkG, for DDR4 memories.\n\n((WL + 1) ×tCkD + tWTR) / tCkG, for LPDDR4 memories."]
    #[inline(always)]
    pub fn wrtord(&self) -> WrtordR {
        WrtordR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Number of cycle between two consecutive Activate commands on\n\ndifferent Banks of the same device.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntRRD / tCkG"]
    #[inline(always)]
    pub fn rrd(&self) -> RrdR {
        RrdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - Field0000 Abstract\n\nNumber of cycle of the FAW period.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntFAW / tCkG"]
    #[inline(always)]
    pub fn faw(&self) -> FawR {
        FawR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Minimum number of scheduler clock cycles between the last DRAM\n\nRead command and a Write command.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\n2 ×tCkD / tCkG, for DDR2 memories.\n\n(RL –WL + 2) ×tCkD / tCKG, for DDR3 and DDR4 memories.\n\n(RL + RoundUp(tDQSCK(max) / tCKD) + tRPST - WL + tWPRE)\n\n×tCkD / tCKG , for LPDDR4 memories."]
    #[inline(always)]
    #[must_use]
    pub fn rdtowr(&mut self) -> RdtowrW<DdrTimingB0Spec> {
        RdtowrW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Minimum number of scheduler clock cycles between the last DRAM\n\nWrite command and a Read command.\n\nThe field must be set to the following value, rounded to an integer\n\nof scheduler clock cycles:\n\n(WL ×tCkD + tWTR) / tCkG, for DDR2 and DDR3 memories.\n\n(WL ×tCkD + tWTR_S) / tCkG, for DDR4 memories.\n\n((WL + 1) ×tCkD + tWTR) / tCkG, for LPDDR4 memories."]
    #[inline(always)]
    #[must_use]
    pub fn wrtord(&mut self) -> WrtordW<DdrTimingB0Spec> {
        WrtordW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Number of cycle between two consecutive Activate commands on\n\ndifferent Banks of the same device.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntRRD / tCkG"]
    #[inline(always)]
    #[must_use]
    pub fn rrd(&mut self) -> RrdW<DdrTimingB0Spec> {
        RrdW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Field0000 Abstract\n\nNumber of cycle of the FAW period.\n\nThe field must be set to the following value, rounded to an integer\n\nnumber of scheduler clock cycles:\n\ntFAW / tCkG"]
    #[inline(always)]
    #[must_use]
    pub fn faw(&mut self) -> FawW<DdrTimingB0Spec> {
        FawW::new(self, 24)
    }
}
#[doc = "DdrTimingB bank 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_timing_b0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_timing_b0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrTimingB0Spec;
impl crate::RegisterSpec for DdrTimingB0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_timing_b0::R`](R) reader structure"]
impl crate::Readable for DdrTimingB0Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_timing_b0::W`](W) writer structure"]
impl crate::Writable for DdrTimingB0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DdrTimingB0 to value 0x0004_0702"]
impl crate::Resettable for DdrTimingB0Spec {
    const RESET_VALUE: u32 = 0x0004_0702;
}
