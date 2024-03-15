#[doc = "Register `MSCH_DdrTimingB0` reader"]
pub type R = crate::R<MschDdrTimingB0Spec>;
#[doc = "Register `MSCH_DdrTimingB0` writer"]
pub type W = crate::W<MschDdrTimingB0Spec>;
#[doc = "Field `RDTOWR` reader - Minimum number of scheduler clock cycles between the last DRAM Read command and a Write command. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: 2 ×tCkD / tCkG, for DDR2 memories. (RL –WL + 2) ×tCkD / tCKG, for DDR3 and DDR4 memories. (RL + RoundUp(tDQSCK(max) / tCKD) + tRPST - WL + tWPRE) ×tCkD / tCKG , for LPDDR4 memories."]
pub type RdtowrR = crate::FieldReader;
#[doc = "Field `RDTOWR` writer - Minimum number of scheduler clock cycles between the last DRAM Read command and a Write command. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: 2 ×tCkD / tCkG, for DDR2 memories. (RL –WL + 2) ×tCkD / tCKG, for DDR3 and DDR4 memories. (RL + RoundUp(tDQSCK(max) / tCKD) + tRPST - WL + tWPRE) ×tCkD / tCKG , for LPDDR4 memories."]
pub type RdtowrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRTORD` reader - Minimum number of scheduler clock cycles between the last DRAM Write command and a Read command. The field must be set to the following value, rounded to an integer of scheduler clock cycles: (WL ×tCkD + tWTR) / tCkG, for DDR2 and DDR3 memories. (WL ×tCkD + tWTR_S) / tCkG, for DDR4 memories. ((WL + 1) ×tCkD + tWTR) / tCkG, for LPDDR4 memories."]
pub type WrtordR = crate::FieldReader;
#[doc = "Field `WRTORD` writer - Minimum number of scheduler clock cycles between the last DRAM Write command and a Read command. The field must be set to the following value, rounded to an integer of scheduler clock cycles: (WL ×tCkD + tWTR) / tCkG, for DDR2 and DDR3 memories. (WL ×tCkD + tWTR_S) / tCkG, for DDR4 memories. ((WL + 1) ×tCkD + tWTR) / tCkG, for LPDDR4 memories."]
pub type WrtordW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RRD` reader - Number of cycle between two consecutive Activate commands on different Banks of the same device. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: tRRD / tCkG"]
pub type RrdR = crate::FieldReader;
#[doc = "Field `RRD` writer - Number of cycle between two consecutive Activate commands on different Banks of the same device. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: tRRD / tCkG"]
pub type RrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FAW` reader - Field0000 Abstract Number of cycle of the FAW period. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: tFAW / tCkG"]
pub type FawR = crate::FieldReader;
#[doc = "Field `FAW` writer - Field0000 Abstract Number of cycle of the FAW period. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: tFAW / tCkG"]
pub type FawW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:4 - Minimum number of scheduler clock cycles between the last DRAM Read command and a Write command. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: 2 ×tCkD / tCkG, for DDR2 memories. (RL –WL + 2) ×tCkD / tCKG, for DDR3 and DDR4 memories. (RL + RoundUp(tDQSCK(max) / tCKD) + tRPST - WL + tWPRE) ×tCkD / tCKG , for LPDDR4 memories."]
    #[inline(always)]
    pub fn rdtowr(&self) -> RdtowrR {
        RdtowrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Minimum number of scheduler clock cycles between the last DRAM Write command and a Read command. The field must be set to the following value, rounded to an integer of scheduler clock cycles: (WL ×tCkD + tWTR) / tCkG, for DDR2 and DDR3 memories. (WL ×tCkD + tWTR_S) / tCkG, for DDR4 memories. ((WL + 1) ×tCkD + tWTR) / tCkG, for LPDDR4 memories."]
    #[inline(always)]
    pub fn wrtord(&self) -> WrtordR {
        WrtordR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Number of cycle between two consecutive Activate commands on different Banks of the same device. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: tRRD / tCkG"]
    #[inline(always)]
    pub fn rrd(&self) -> RrdR {
        RrdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - Field0000 Abstract Number of cycle of the FAW period. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: tFAW / tCkG"]
    #[inline(always)]
    pub fn faw(&self) -> FawR {
        FawR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Minimum number of scheduler clock cycles between the last DRAM Read command and a Write command. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: 2 ×tCkD / tCkG, for DDR2 memories. (RL –WL + 2) ×tCkD / tCKG, for DDR3 and DDR4 memories. (RL + RoundUp(tDQSCK(max) / tCKD) + tRPST - WL + tWPRE) ×tCkD / tCKG , for LPDDR4 memories."]
    #[inline(always)]
    #[must_use]
    pub fn rdtowr(&mut self) -> RdtowrW<MschDdrTimingB0Spec> {
        RdtowrW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Minimum number of scheduler clock cycles between the last DRAM Write command and a Read command. The field must be set to the following value, rounded to an integer of scheduler clock cycles: (WL ×tCkD + tWTR) / tCkG, for DDR2 and DDR3 memories. (WL ×tCkD + tWTR_S) / tCkG, for DDR4 memories. ((WL + 1) ×tCkD + tWTR) / tCkG, for LPDDR4 memories."]
    #[inline(always)]
    #[must_use]
    pub fn wrtord(&mut self) -> WrtordW<MschDdrTimingB0Spec> {
        WrtordW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Number of cycle between two consecutive Activate commands on different Banks of the same device. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: tRRD / tCkG"]
    #[inline(always)]
    #[must_use]
    pub fn rrd(&mut self) -> RrdW<MschDdrTimingB0Spec> {
        RrdW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Field0000 Abstract Number of cycle of the FAW period. The field must be set to the following value, rounded to an integer number of scheduler clock cycles: tFAW / tCkG"]
    #[inline(always)]
    #[must_use]
    pub fn faw(&mut self) -> FawW<MschDdrTimingB0Spec> {
        FawW::new(self, 24)
    }
}
#[doc = "DdrTimingB bank 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_ddr_timing_b0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_ddr_timing_b0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MschDdrTimingB0Spec;
impl crate::RegisterSpec for MschDdrTimingB0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msch_ddr_timing_b0::R`](R) reader structure"]
impl crate::Readable for MschDdrTimingB0Spec {}
#[doc = "`write(|w| ..)` method takes [`msch_ddr_timing_b0::W`](W) writer structure"]
impl crate::Writable for MschDdrTimingB0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSCH_DdrTimingB0 to value 0x0004_0702"]
impl crate::Resettable for MschDdrTimingB0Spec {
    const RESET_VALUE: u32 = 0x0004_0702;
}
