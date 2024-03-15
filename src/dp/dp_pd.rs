#[doc = "Register `DP_PD` reader"]
pub type R = crate::R<DpPdSpec>;
#[doc = "Register `DP_PD` writer"]
pub type W = crate::W<DpPdSpec>;
#[doc = "Field `PD_CH0` reader - Power down ch0"]
pub type PdCh0R = crate::BitReader;
#[doc = "Field `PD_CH0` writer - Power down ch0"]
pub type PdCh0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PD_CH1` reader - Power down ch1"]
pub type PdCh1R = crate::BitReader;
#[doc = "Field `PD_CH1` writer - Power down ch1"]
pub type PdCh1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PD_CH2` reader - Power down ch2"]
pub type PdCh2R = crate::BitReader;
#[doc = "Field `PD_CH2` writer - Power down ch2"]
pub type PdCh2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PD_CH3` reader - Power down ch3"]
pub type PdCh3R = crate::BitReader;
#[doc = "Field `PD_CH3` writer - Power down ch3"]
pub type PdCh3W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PD_PLL` reader - Power down PLL"]
pub type PdPllR = crate::BitReader;
#[doc = "Field `PD_PLL` writer - Power down PLL"]
pub type PdPllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_AUX_CH` reader - Power down AUX channel"]
pub type PdAuxChR = crate::BitReader;
#[doc = "Field `PD_AUX_CH` writer - Power down AUX channel"]
pub type PdAuxChW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PD_EXP_BG` reader - Power down all except band gap"]
pub type PdExpBgR = crate::BitReader;
#[doc = "Field `PD_EXP_BG` writer - Power down all except band gap"]
pub type PdExpBgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_INC_BG` reader - Power down all including band gap"]
pub type PdIncBgR = crate::BitReader;
#[doc = "Field `PD_INC_BG` writer - Power down all including band gap"]
pub type PdIncBgW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power down ch0"]
    #[inline(always)]
    pub fn pd_ch0(&self) -> PdCh0R {
        PdCh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down ch1"]
    #[inline(always)]
    pub fn pd_ch1(&self) -> PdCh1R {
        PdCh1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power down ch2"]
    #[inline(always)]
    pub fn pd_ch2(&self) -> PdCh2R {
        PdCh2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power down ch3"]
    #[inline(always)]
    pub fn pd_ch3(&self) -> PdCh3R {
        PdCh3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power down PLL"]
    #[inline(always)]
    pub fn pd_pll(&self) -> PdPllR {
        PdPllR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Power down AUX channel"]
    #[inline(always)]
    pub fn pd_aux_ch(&self) -> PdAuxChR {
        PdAuxChR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Power down all except band gap"]
    #[inline(always)]
    pub fn pd_exp_bg(&self) -> PdExpBgR {
        PdExpBgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power down all including band gap"]
    #[inline(always)]
    pub fn pd_inc_bg(&self) -> PdIncBgR {
        PdIncBgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down ch0"]
    #[inline(always)]
    #[must_use]
    pub fn pd_ch0(&mut self) -> PdCh0W<DpPdSpec> {
        PdCh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Power down ch1"]
    #[inline(always)]
    #[must_use]
    pub fn pd_ch1(&mut self) -> PdCh1W<DpPdSpec> {
        PdCh1W::new(self, 1)
    }
    #[doc = "Bit 2 - Power down ch2"]
    #[inline(always)]
    #[must_use]
    pub fn pd_ch2(&mut self) -> PdCh2W<DpPdSpec> {
        PdCh2W::new(self, 2)
    }
    #[doc = "Bit 3 - Power down ch3"]
    #[inline(always)]
    #[must_use]
    pub fn pd_ch3(&mut self) -> PdCh3W<DpPdSpec> {
        PdCh3W::new(self, 3)
    }
    #[doc = "Bit 4 - Power down PLL"]
    #[inline(always)]
    #[must_use]
    pub fn pd_pll(&mut self) -> PdPllW<DpPdSpec> {
        PdPllW::new(self, 4)
    }
    #[doc = "Bit 5 - Power down AUX channel"]
    #[inline(always)]
    #[must_use]
    pub fn pd_aux_ch(&mut self) -> PdAuxChW<DpPdSpec> {
        PdAuxChW::new(self, 5)
    }
    #[doc = "Bit 6 - Power down all except band gap"]
    #[inline(always)]
    #[must_use]
    pub fn pd_exp_bg(&mut self) -> PdExpBgW<DpPdSpec> {
        PdExpBgW::new(self, 6)
    }
    #[doc = "Bit 7 - Power down all including band gap"]
    #[inline(always)]
    #[must_use]
    pub fn pd_inc_bg(&mut self) -> PdIncBgW<DpPdSpec> {
        PdIncBgW::new(self, 7)
    }
}
#[doc = "Power down control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_pd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_pd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpPdSpec;
impl crate::RegisterSpec for DpPdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_pd::R`](R) reader structure"]
impl crate::Readable for DpPdSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_pd::W`](W) writer structure"]
impl crate::Writable for DpPdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xaf;
}
#[doc = "`reset()` method sets DP_PD to value 0xff"]
impl crate::Resettable for DpPdSpec {
    const RESET_VALUE: u32 = 0xff;
}
