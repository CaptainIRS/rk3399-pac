#[doc = "Register `SWREG83` reader"]
pub type R = crate::R<Swreg83Spec>;
#[doc = "Register `SWREG83` writer"]
pub type W = crate::W<Swreg83Spec>;
#[doc = "Field `H264_NUM_REF_IDX14` reader - the number of referance pic index14"]
pub type H264NumRefIdx14R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX14` writer - the number of referance pic index14"]
pub type H264NumRefIdx14W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_NUM_REF_IDX15` reader - the number of referance pic index15"]
pub type H264NumRefIdx15R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX15` writer - the number of referance pic index15"]
pub type H264NumRefIdx15W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of referance pic index14"]
    #[inline(always)]
    pub fn h264_num_ref_idx14(&self) -> H264NumRefIdx14R {
        H264NumRefIdx14R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the number of referance pic index15"]
    #[inline(always)]
    pub fn h264_num_ref_idx15(&self) -> H264NumRefIdx15R {
        H264NumRefIdx15R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of referance pic index14"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx14(&mut self) -> H264NumRefIdx14W<Swreg83Spec> {
        H264NumRefIdx14W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the number of referance pic index15"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx15(&mut self) -> H264NumRefIdx15W<Swreg83Spec> {
        H264NumRefIdx15W::new(self, 16)
    }
}
#[doc = "the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg83::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg83::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg83Spec;
impl crate::RegisterSpec for Swreg83Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg83::R`](R) reader structure"]
impl crate::Readable for Swreg83Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg83::W`](W) writer structure"]
impl crate::Writable for Swreg83Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG83 to value 0"]
impl crate::Resettable for Swreg83Spec {
    const RESET_VALUE: u32 = 0;
}
