#[doc = "Register `DENALI_CTL_179` reader"]
pub type R = crate::R<DenaliCtl179Spec>;
#[doc = "Register `DENALI_CTL_179` writer"]
pub type W = crate::W<DenaliCtl179Spec>;
#[doc = "Field `ZQINIT_F0` reader - Number of cycles needed for a ZQINIT command for frequency copy 0."]
pub type ZqinitF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQINIT_F0` writer - Number of cycles needed for a ZQINIT command for frequency copy 0."]
pub type ZqinitF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 8:19 - Number of cycles needed for a ZQINIT command for frequency copy 0."]
    #[inline(always)]
    pub fn zqinit_f0(&self) -> ZqinitF0R {
        ZqinitF0R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:19 - Number of cycles needed for a ZQINIT command for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn zqinit_f0(&mut self) -> ZqinitF0W<DenaliCtl179Spec> {
        ZqinitF0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_179::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_179::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl179Spec;
impl crate::RegisterSpec for DenaliCtl179Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_179::R`](R) reader structure"]
impl crate::Readable for DenaliCtl179Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_179::W`](W) writer structure"]
impl crate::Writable for DenaliCtl179Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_179 to value 0"]
impl crate::Resettable for DenaliCtl179Spec {
    const RESET_VALUE: u32 = 0;
}
