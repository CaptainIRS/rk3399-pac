#[doc = "Register `SWREG_58` reader"]
pub type R = crate::R<Swreg58Spec>;
#[doc = "Register `SWREG_58` writer"]
pub type W = crate::W<Swreg58Spec>;
#[doc = "Field `QP_SUM_DIV2` reader - the result of (qp sum)/2\n\nthe result of (qp sum)/2"]
pub type QpSumDiv2R = crate::FieldReader<u32>;
#[doc = "Field `QP_SUM_DIV2` writer - the result of (qp sum)/2\n\nthe result of (qp sum)/2"]
pub type QpSumDiv2W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 11:31 - the result of (qp sum)/2\n\nthe result of (qp sum)/2"]
    #[inline(always)]
    pub fn qp_sum_div2(&self) -> QpSumDiv2R {
        QpSumDiv2R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 11:31 - the result of (qp sum)/2\n\nthe result of (qp sum)/2"]
    #[inline(always)]
    #[must_use]
    pub fn qp_sum_div2(&mut self) -> QpSumDiv2W<Swreg58Spec> {
        QpSumDiv2W::new(self, 11)
    }
}
#[doc = "the result of qp sum div2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_58::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_58::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg58Spec;
impl crate::RegisterSpec for Swreg58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_58::R`](R) reader structure"]
impl crate::Readable for Swreg58Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_58::W`](W) writer structure"]
impl crate::Writable for Swreg58Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_58 to value 0"]
impl crate::Resettable for Swreg58Spec {
    const RESET_VALUE: u32 = 0;
}
