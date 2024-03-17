#[doc = "Register `DENALI_CTL_40` reader"]
pub type R = crate::R<DenaliCtl40Spec>;
#[doc = "Register `DENALI_CTL_40` writer"]
pub type W = crate::W<DenaliCtl40Spec>;
#[doc = "Field `TWR_F1` reader - DRAM TWR value for frequency copy 1 in cycles."]
pub type TwrF1R = crate::FieldReader;
#[doc = "Field `TWR_F1` writer - DRAM TWR value for frequency copy 1 in cycles."]
pub type TwrF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRCD_F2` reader - DRAM TRCD value for frequency copy 2 in cycles."]
pub type TrcdF2R = crate::FieldReader;
#[doc = "Field `TRCD_F2` writer - DRAM TRCD value for frequency copy 2 in cycles."]
pub type TrcdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TWR_F2` reader - DRAM TWR value for frequency copy 2 in cycles."]
pub type TwrF2R = crate::FieldReader;
#[doc = "Field `TWR_F2` writer - DRAM TWR value for frequency copy 2 in cycles."]
pub type TwrF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TMRR` reader - DRAM TMRR value in cycles."]
pub type TmrrR = crate::FieldReader;
#[doc = "Field `TMRR` writer - DRAM TMRR value in cycles."]
pub type TmrrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - DRAM TWR value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn twr_f1(&self) -> TwrF1R {
        TwrF1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TRCD value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn trcd_f2(&self) -> TrcdF2R {
        TrcdF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - DRAM TWR value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn twr_f2(&self) -> TwrF2R {
        TwrF2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - DRAM TMRR value in cycles."]
    #[inline(always)]
    pub fn tmrr(&self) -> TmrrR {
        TmrrR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DRAM TWR value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn twr_f1(&mut self) -> TwrF1W<DenaliCtl40Spec> {
        TwrF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TRCD value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trcd_f2(&mut self) -> TrcdF2W<DenaliCtl40Spec> {
        TrcdF2W::new(self, 8)
    }
    #[doc = "Bits 16:21 - DRAM TWR value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn twr_f2(&mut self) -> TwrF2W<DenaliCtl40Spec> {
        TwrF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DRAM TMRR value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrr(&mut self) -> TmrrW<DenaliCtl40Spec> {
        TmrrW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_40::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_40::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl40Spec;
impl crate::RegisterSpec for DenaliCtl40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_40::R`](R) reader structure"]
impl crate::Readable for DenaliCtl40Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_40::W`](W) writer structure"]
impl crate::Writable for DenaliCtl40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_40 to value 0"]
impl crate::Resettable for DenaliCtl40Spec {
    const RESET_VALUE: u32 = 0;
}
