#[doc = "Register `PMUPVTM_CON0` reader"]
pub type R = crate::R<PmupvtmCon0Spec>;
#[doc = "Register `PMUPVTM_CON0` writer"]
pub type W = crate::W<PmupvtmCon0Spec>;
#[doc = "Field `PVTM_START` reader - pmu pvtm start"]
pub type PvtmStartR = crate::BitReader;
#[doc = "Field `PVTM_START` writer - pmu pvtm start"]
pub type PvtmStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVTM_OSC_EN` reader - pmu pvtm osc enable"]
pub type PvtmOscEnR = crate::BitReader;
#[doc = "Field `PVTM_OSC_EN` writer - pmu pvtm osc enable"]
pub type PvtmOscEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVTM_CLKOUT_DIV` reader - clk_pvtm_out_div=clk_pvtm_out/pvtm_clko\n\nut_div"]
pub type PvtmClkoutDivR = crate::FieldReader<u16>;
#[doc = "Field `PVTM_CLKOUT_DIV` writer - clk_pvtm_out_div=clk_pvtm_out/pvtm_clko\n\nut_div"]
pub type PvtmClkoutDivW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `WRITE_ENABLE` reader - When bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - When bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pmu pvtm start"]
    #[inline(always)]
    pub fn pvtm_start(&self) -> PvtmStartR {
        PvtmStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pmu pvtm osc enable"]
    #[inline(always)]
    pub fn pvtm_osc_en(&self) -> PvtmOscEnR {
        PvtmOscEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - clk_pvtm_out_div=clk_pvtm_out/pvtm_clko\n\nut_div"]
    #[inline(always)]
    pub fn pvtm_clkout_div(&self) -> PvtmClkoutDivR {
        PvtmClkoutDivR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:31 - When bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - pmu pvtm start"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_start(&mut self) -> PvtmStartW<PmupvtmCon0Spec> {
        PvtmStartW::new(self, 0)
    }
    #[doc = "Bit 1 - pmu pvtm osc enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_osc_en(&mut self) -> PvtmOscEnW<PmupvtmCon0Spec> {
        PvtmOscEnW::new(self, 1)
    }
    #[doc = "Bits 2:15 - clk_pvtm_out_div=clk_pvtm_out/pvtm_clko\n\nut_div"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_clkout_div(&mut self) -> PvtmClkoutDivW<PmupvtmCon0Spec> {
        PvtmClkoutDivW::new(self, 2)
    }
    #[doc = "Bits 16:31 - When bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmupvtmCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "pmu pvtm configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmupvtm_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmupvtm_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmupvtmCon0Spec;
impl crate::RegisterSpec for PmupvtmCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmupvtm_con0::R`](R) reader structure"]
impl crate::Readable for PmupvtmCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmupvtm_con0::W`](W) writer structure"]
impl crate::Writable for PmupvtmCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUPVTM_CON0 to value 0"]
impl crate::Resettable for PmupvtmCon0Spec {
    const RESET_VALUE: u32 = 0;
}
