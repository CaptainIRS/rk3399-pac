#[doc = "Register `SWREG80` reader"]
pub type R = crate::R<Swreg80Spec>;
#[doc = "Register `SWREG80` writer"]
pub type W = crate::W<Swreg80Spec>;
#[doc = "Field `H264_NUM_REF_IDX8` reader - the number of referance pic index8"]
pub type H264NumRefIdx8R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX8` writer - the number of referance pic index8"]
pub type H264NumRefIdx8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_NUM_REF_IDX9` reader - the number of referance pic index9"]
pub type H264NumRefIdx9R = crate::FieldReader<u16>;
#[doc = "Field `H264_NUM_REF_IDX9` writer - the number of referance pic index9"]
pub type H264NumRefIdx9W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the number of referance pic index8"]
    #[inline(always)]
    pub fn h264_num_ref_idx8(&self) -> H264NumRefIdx8R {
        H264NumRefIdx8R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the number of referance pic index9"]
    #[inline(always)]
    pub fn h264_num_ref_idx9(&self) -> H264NumRefIdx9R {
        H264NumRefIdx9R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - the number of referance pic index8"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx8(&mut self) -> H264NumRefIdx8W<Swreg80Spec> {
        H264NumRefIdx8W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the number of referance pic index9"]
    #[inline(always)]
    #[must_use]
    pub fn h264_num_ref_idx9(&mut self) -> H264NumRefIdx9W<Swreg80Spec> {
        H264NumRefIdx9W::new(self, 16)
    }
}
#[doc = "the number of referance pic\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg80::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg80::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg80Spec;
impl crate::RegisterSpec for Swreg80Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg80::R`](R) reader structure"]
impl crate::Readable for Swreg80Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg80::W`](W) writer structure"]
impl crate::Writable for Swreg80Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG80 to value 0"]
impl crate::Resettable for Swreg80Spec {
    const RESET_VALUE: u32 = 0;
}
