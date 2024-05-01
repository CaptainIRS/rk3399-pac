#[doc = "Register `SWREG_67_REUSE` reader"]
pub type R = crate::R<Swreg67ReuseSpec>;
#[doc = "Register `SWREG_67_REUSE` writer"]
pub type W = crate::W<Swreg67ReuseSpec>;
#[doc = "Field `H264_CHKPT_6` reader - 6st word used for check point used in h.264\n\n6st word used for check point used in h.264"]
pub type H264Chkpt6R = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_6` writer - 6st word used for check point used in h.264\n\n6st word used for check point used in h.264"]
pub type H264Chkpt6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_CHKPT_5` reader - 5st word used for check point used in h.264\n\n5st word used for check point used in h.264"]
pub type H264Chkpt5R = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_5` writer - 5st word used for check point used in h.264\n\n5st word used for check point used in h.264"]
pub type H264Chkpt5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 6st word used for check point used in h.264\n\n6st word used for check point used in h.264"]
    #[inline(always)]
    pub fn h264_chkpt_6(&self) -> H264Chkpt6R {
        H264Chkpt6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 5st word used for check point used in h.264\n\n5st word used for check point used in h.264"]
    #[inline(always)]
    pub fn h264_chkpt_5(&self) -> H264Chkpt5R {
        H264Chkpt5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 6st word used for check point used in h.264\n\n6st word used for check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_6(&mut self) -> H264Chkpt6W<Swreg67ReuseSpec> {
        H264Chkpt6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 5st word used for check point used in h.264\n\n5st word used for check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_5(&mut self) -> H264Chkpt5W<Swreg67ReuseSpec> {
        H264Chkpt5W::new(self, 16)
    }
}
#[doc = "checkpoint 5 and 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_67_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_67_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg67ReuseSpec;
impl crate::RegisterSpec for Swreg67ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_67_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg67ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_67_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg67ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_67_REUSE to value 0"]
impl crate::Resettable for Swreg67ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
