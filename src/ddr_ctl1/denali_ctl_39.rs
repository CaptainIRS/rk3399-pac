#[doc = "Register `DENALI_CTL_39` reader"]
pub type R = crate::R<DenaliCtl39Spec>;
#[doc = "Register `DENALI_CTL_39` writer"]
pub type W = crate::W<DenaliCtl39Spec>;
#[doc = "Field `WRITEINTERP` reader - Allow controller to interrupt a write burst to the DRAMs with a read command. Set to 1 to allow interruption."]
pub type WriteinterpR = crate::BitReader;
#[doc = "Field `WRITEINTERP` writer - Allow controller to interrupt a write burst to the DRAMs with a read command. Set to 1 to allow interruption."]
pub type WriteinterpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCD_F0` reader - DRAM TRCD value for frequency copy 0 in cycles."]
pub type TrcdF0R = crate::FieldReader;
#[doc = "Field `TRCD_F0` writer - DRAM TRCD value for frequency copy 0 in cycles."]
pub type TrcdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TWR_F0` reader - DRAM TWR value for frequency copy 0 in cycles."]
pub type TwrF0R = crate::FieldReader;
#[doc = "Field `TWR_F0` writer - DRAM TWR value for frequency copy 0 in cycles."]
pub type TwrF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRCD_F1` reader - DRAM TRCD value for frequency copy 1 in cycles."]
pub type TrcdF1R = crate::FieldReader;
#[doc = "Field `TRCD_F1` writer - DRAM TRCD value for frequency copy 1 in cycles."]
pub type TrcdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Allow controller to interrupt a write burst to the DRAMs with a read command. Set to 1 to allow interruption."]
    #[inline(always)]
    pub fn writeinterp(&self) -> WriteinterpR {
        WriteinterpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - DRAM TRCD value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn trcd_f0(&self) -> TrcdF0R {
        TrcdF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - DRAM TWR value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn twr_f0(&self) -> TwrF0R {
        TwrF0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - DRAM TRCD value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn trcd_f1(&self) -> TrcdF1R {
        TrcdF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Allow controller to interrupt a write burst to the DRAMs with a read command. Set to 1 to allow interruption."]
    #[inline(always)]
    #[must_use]
    pub fn writeinterp(&mut self) -> WriteinterpW<DenaliCtl39Spec> {
        WriteinterpW::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TRCD value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trcd_f0(&mut self) -> TrcdF0W<DenaliCtl39Spec> {
        TrcdF0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - DRAM TWR value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn twr_f0(&mut self) -> TwrF0W<DenaliCtl39Spec> {
        TwrF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DRAM TRCD value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trcd_f1(&mut self) -> TrcdF1W<DenaliCtl39Spec> {
        TrcdF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl39Spec;
impl crate::RegisterSpec for DenaliCtl39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_39::R`](R) reader structure"]
impl crate::Readable for DenaliCtl39Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_39::W`](W) writer structure"]
impl crate::Writable for DenaliCtl39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_39 to value 0"]
impl crate::Resettable for DenaliCtl39Spec {
    const RESET_VALUE: u32 = 0;
}
