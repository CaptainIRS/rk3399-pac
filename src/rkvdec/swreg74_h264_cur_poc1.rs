#[doc = "Register `SWREG74_H264_CUR_POC1` reader"]
pub type R = crate::R<Swreg74H264CurPoc1Spec>;
#[doc = "Register `SWREG74_H264_CUR_POC1` writer"]
pub type W = crate::W<Swreg74H264CurPoc1Spec>;
#[doc = "Field `SW_H264_CUR_POC1` reader - h264 cur poc for bottom field\n\nh264 cur poc for bottom field"]
pub type SwH264CurPoc1R = crate::FieldReader<u32>;
#[doc = "Field `SW_H264_CUR_POC1` writer - h264 cur poc for bottom field\n\nh264 cur poc for bottom field"]
pub type SwH264CurPoc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - h264 cur poc for bottom field\n\nh264 cur poc for bottom field"]
    #[inline(always)]
    pub fn sw_h264_cur_poc1(&self) -> SwH264CurPoc1R {
        SwH264CurPoc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - h264 cur poc for bottom field\n\nh264 cur poc for bottom field"]
    #[inline(always)]
    #[must_use]
    pub fn sw_h264_cur_poc1(&mut self) -> SwH264CurPoc1W<Swreg74H264CurPoc1Spec> {
        SwH264CurPoc1W::new(self, 0)
    }
}
#[doc = "h264 cur poc for bottom filed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg74_h264_cur_poc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg74_h264_cur_poc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg74H264CurPoc1Spec;
impl crate::RegisterSpec for Swreg74H264CurPoc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg74_h264_cur_poc1::R`](R) reader structure"]
impl crate::Readable for Swreg74H264CurPoc1Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg74_h264_cur_poc1::W`](W) writer structure"]
impl crate::Writable for Swreg74H264CurPoc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG74_H264_CUR_POC1 to value 0"]
impl crate::Resettable for Swreg74H264CurPoc1Spec {
    const RESET_VALUE: u32 = 0;
}
