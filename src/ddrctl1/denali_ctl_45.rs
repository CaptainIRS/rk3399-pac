#[doc = "Register `DENALI_CTL_45` reader"]
pub type R = crate::R<DenaliCtl45Spec>;
#[doc = "Register `DENALI_CTL_45` writer"]
pub type W = crate::W<DenaliCtl45Spec>;
#[doc = "Field `TRP_AB_F0` reader - DRAM TRP all bank value for frequency copy 0 in cycles."]
pub type TrpAbF0R = crate::FieldReader;
#[doc = "Field `TRP_AB_F0` writer - DRAM TRP all bank value for frequency copy 0 in cycles."]
pub type TrpAbF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRP_AB_F1` reader - DRAM TRP all bank value for frequency copy 1 in cycles."]
pub type TrpAbF1R = crate::FieldReader;
#[doc = "Field `TRP_AB_F1` writer - DRAM TRP all bank value for frequency copy 1 in cycles."]
pub type TrpAbF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRP_AB_F2` reader - DRAM TRP all bank value for frequency copy 2 in cycles."]
pub type TrpAbF2R = crate::FieldReader;
#[doc = "Field `TRP_AB_F2` writer - DRAM TRP all bank value for frequency copy 2 in cycles."]
pub type TrpAbF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DIMM_ENABLE` reader - Enable registered DIMM operation of the controller. Set to 1 to enable."]
pub type RegDimmEnableR = crate::BitReader;
#[doc = "Field `REG_DIMM_ENABLE` writer - Enable registered DIMM operation of the controller. Set to 1 to enable."]
pub type RegDimmEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - DRAM TRP all bank value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn trp_ab_f0(&self) -> TrpAbF0R {
        TrpAbF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TRP all bank value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn trp_ab_f1(&self) -> TrpAbF1R {
        TrpAbF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DRAM TRP all bank value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn trp_ab_f2(&self) -> TrpAbF2R {
        TrpAbF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable registered DIMM operation of the controller. Set to 1 to enable."]
    #[inline(always)]
    pub fn reg_dimm_enable(&self) -> RegDimmEnableR {
        RegDimmEnableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DRAM TRP all bank value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trp_ab_f0(&mut self) -> TrpAbF0W<DenaliCtl45Spec> {
        TrpAbF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TRP all bank value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trp_ab_f1(&mut self) -> TrpAbF1W<DenaliCtl45Spec> {
        TrpAbF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DRAM TRP all bank value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trp_ab_f2(&mut self) -> TrpAbF2W<DenaliCtl45Spec> {
        TrpAbF2W::new(self, 16)
    }
    #[doc = "Bit 24 - Enable registered DIMM operation of the controller. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn reg_dimm_enable(&mut self) -> RegDimmEnableW<DenaliCtl45Spec> {
        RegDimmEnableW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl45Spec;
impl crate::RegisterSpec for DenaliCtl45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_45::R`](R) reader structure"]
impl crate::Readable for DenaliCtl45Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_45::W`](W) writer structure"]
impl crate::Writable for DenaliCtl45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_45 to value 0"]
impl crate::Resettable for DenaliCtl45Spec {
    const RESET_VALUE: u32 = 0;
}
