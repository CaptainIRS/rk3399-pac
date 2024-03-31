#[doc = "Register `DENALI_CTL_93` reader"]
pub type R = crate::R<DenaliCtl93Spec>;
#[doc = "Register `DENALI_CTL_93` writer"]
pub type W = crate::W<DenaliCtl93Spec>;
#[doc = "Field `CKSRX_F1` reader - Clock stable delay on self-refresh exit for frequency copy 1."]
pub type CksrxF1R = crate::FieldReader;
#[doc = "Field `CKSRX_F1` writer - Clock stable delay on self-refresh exit for frequency copy 1."]
pub type CksrxF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKSRE_F2` reader - Clock hold delay on self-refresh entry for frequency copy 2."]
pub type CksreF2R = crate::FieldReader;
#[doc = "Field `CKSRE_F2` writer - Clock hold delay on self-refresh entry for frequency copy 2."]
pub type CksreF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKSRX_F2` reader - Clock stable delay on self-refresh exit for frequency copy 2."]
pub type CksrxF2R = crate::FieldReader;
#[doc = "Field `CKSRX_F2` writer - Clock stable delay on self-refresh exit for frequency copy 2."]
pub type CksrxF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_CMD` writer - Low power software command request interface. Bit (0) controls exit, bit (1) controls entry, bits (4:2) define the low power state, bit (5) controls memory clock gating, bit (6) controls controller clock gating, and bit (7) controls lock."]
pub type LpCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock stable delay on self-refresh exit for frequency copy 1."]
    #[inline(always)]
    pub fn cksrx_f1(&self) -> CksrxF1R {
        CksrxF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock hold delay on self-refresh entry for frequency copy 2."]
    #[inline(always)]
    pub fn cksre_f2(&self) -> CksreF2R {
        CksreF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock stable delay on self-refresh exit for frequency copy 2."]
    #[inline(always)]
    pub fn cksrx_f2(&self) -> CksrxF2R {
        CksrxF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock stable delay on self-refresh exit for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn cksrx_f1(&mut self) -> CksrxF1W<DenaliCtl93Spec> {
        CksrxF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock hold delay on self-refresh entry for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn cksre_f2(&mut self) -> CksreF2W<DenaliCtl93Spec> {
        CksreF2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Clock stable delay on self-refresh exit for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn cksrx_f2(&mut self) -> CksrxF2W<DenaliCtl93Spec> {
        CksrxF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Low power software command request interface. Bit (0) controls exit, bit (1) controls entry, bits (4:2) define the low power state, bit (5) controls memory clock gating, bit (6) controls controller clock gating, and bit (7) controls lock."]
    #[inline(always)]
    #[must_use]
    pub fn lp_cmd(&mut self) -> LpCmdW<DenaliCtl93Spec> {
        LpCmdW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_93::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_93::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl93Spec;
impl crate::RegisterSpec for DenaliCtl93Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_93::R`](R) reader structure"]
impl crate::Readable for DenaliCtl93Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_93::W`](W) writer structure"]
impl crate::Writable for DenaliCtl93Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_93 to value 0"]
impl crate::Resettable for DenaliCtl93Spec {
    const RESET_VALUE: u32 = 0;
}
