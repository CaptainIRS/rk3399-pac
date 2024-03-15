#[doc = "Register `BUF_DATA_%s` reader"]
pub type R = crate::R<BufData_Spec>;
#[doc = "Register `BUF_DATA_%s` writer"]
pub type W = crate::W<BufData_Spec>;
#[doc = "Field `BUF_DATA_0_BUF_DATA_15` reader - AUX CH buffer data 0 ~ 15"]
pub type BufData0BufData15R = crate::FieldReader;
#[doc = "Field `BUF_DATA_0_BUF_DATA_15` writer - AUX CH buffer data 0 ~ 15"]
pub type BufData0BufData15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - AUX CH buffer data 0 ~ 15"]
    #[inline(always)]
    pub fn buf_data_0_buf_data_15(&self) -> BufData0BufData15R {
        BufData0BufData15R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AUX CH buffer data 0 ~ 15"]
    #[inline(always)]
    #[must_use]
    pub fn buf_data_0_buf_data_15(&mut self) -> BufData0BufData15W<BufData_Spec> {
        BufData0BufData15W::new(self, 0)
    }
}
#[doc = "AUX CH buffer data 0 ~ 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_data_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_data_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufData_Spec;
impl crate::RegisterSpec for BufData_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_data_::R`](R) reader structure"]
impl crate::Readable for BufData_Spec {}
#[doc = "`write(|w| ..)` method takes [`buf_data_::W`](W) writer structure"]
impl crate::Writable for BufData_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF_DATA_%s to value 0xff"]
impl crate::Resettable for BufData_Spec {
    const RESET_VALUE: u32 = 0xff;
}
