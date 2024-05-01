#[doc = "Register `TDES_KEY3_1` reader"]
pub type R = crate::R<TdesKey3_1Spec>;
#[doc = "Register `TDES_KEY3_1` writer"]
pub type W = crate::W<TdesKey3_1Spec>;
#[doc = "Field `AES_KEY3_1` reader - Specifies TDES key3 data \\[31:0\\]"]
pub type AesKey3_1R = crate::FieldReader<u32>;
#[doc = "Field `AES_KEY3_1` writer - Specifies TDES key3 data \\[31:0\\]"]
pub type AesKey3_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies TDES key3 data \\[31:0\\]"]
    #[inline(always)]
    pub fn aes_key3_1(&self) -> AesKey3_1R {
        AesKey3_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies TDES key3 data \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn aes_key3_1(&mut self) -> AesKey3_1W<TdesKey3_1Spec> {
        AesKey3_1W::new(self, 0)
    }
}
#[doc = "TDES Key3 data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_key3_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_key3_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdesKey3_1Spec;
impl crate::RegisterSpec for TdesKey3_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdes_key3_1::R`](R) reader structure"]
impl crate::Readable for TdesKey3_1Spec {}
#[doc = "`write(|w| ..)` method takes [`tdes_key3_1::W`](W) writer structure"]
impl crate::Writable for TdesKey3_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDES_KEY3_1 to value 0"]
impl crate::Resettable for TdesKey3_1Spec {
    const RESET_VALUE: u32 = 0;
}
