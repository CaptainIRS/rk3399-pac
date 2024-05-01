#[doc = "Register `SWREG_68_REUSE` reader"]
pub type R = crate::R<Swreg68ReuseSpec>;
#[doc = "Register `SWREG_68_REUSE` writer"]
pub type W = crate::W<Swreg68ReuseSpec>;
#[doc = "Field `H264_CHKPT_8` reader - 8st word used for check point used in h.264\n\n8st word used for check point used in h.264"]
pub type H264Chkpt8R = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_8` writer - 8st word used for check point used in h.264\n\n8st word used for check point used in h.264"]
pub type H264Chkpt8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_CHKPT_7` reader - 7st word used for check point used in h.264\n\n7st word used for check point used in h.264"]
pub type H264Chkpt7R = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_7` writer - 7st word used for check point used in h.264\n\n7st word used for check point used in h.264"]
pub type H264Chkpt7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 8st word used for check point used in h.264\n\n8st word used for check point used in h.264"]
    #[inline(always)]
    pub fn h264_chkpt_8(&self) -> H264Chkpt8R {
        H264Chkpt8R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 7st word used for check point used in h.264\n\n7st word used for check point used in h.264"]
    #[inline(always)]
    pub fn h264_chkpt_7(&self) -> H264Chkpt7R {
        H264Chkpt7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 8st word used for check point used in h.264\n\n8st word used for check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_8(&mut self) -> H264Chkpt8W<Swreg68ReuseSpec> {
        H264Chkpt8W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 7st word used for check point used in h.264\n\n7st word used for check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_7(&mut self) -> H264Chkpt7W<Swreg68ReuseSpec> {
        H264Chkpt7W::new(self, 16)
    }
}
#[doc = "checkpoint 7 and 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_68_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_68_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg68ReuseSpec;
impl crate::RegisterSpec for Swreg68ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_68_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg68ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_68_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg68ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_68_REUSE to value 0"]
impl crate::Resettable for Swreg68ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
