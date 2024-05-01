#[doc = "Register `SWREG_69_REUSE` reader"]
pub type R = crate::R<Swreg69ReuseSpec>;
#[doc = "Register `SWREG_69_REUSE` writer"]
pub type W = crate::W<Swreg69ReuseSpec>;
#[doc = "Field `H264_CHKPT_10` reader - 10st word used for check point used in h.264\n\n10st word used for check point used in h.264"]
pub type H264Chkpt10R = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_10` writer - 10st word used for check point used in h.264\n\n10st word used for check point used in h.264"]
pub type H264Chkpt10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `H264_CHKPT_9` reader - 9st word used for check point used in h.264\n\n9st word used for check point used in h.264"]
pub type H264Chkpt9R = crate::FieldReader<u16>;
#[doc = "Field `H264_CHKPT_9` writer - 9st word used for check point used in h.264\n\n9st word used for check point used in h.264"]
pub type H264Chkpt9W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 10st word used for check point used in h.264\n\n10st word used for check point used in h.264"]
    #[inline(always)]
    pub fn h264_chkpt_10(&self) -> H264Chkpt10R {
        H264Chkpt10R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 9st word used for check point used in h.264\n\n9st word used for check point used in h.264"]
    #[inline(always)]
    pub fn h264_chkpt_9(&self) -> H264Chkpt9R {
        H264Chkpt9R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 10st word used for check point used in h.264\n\n10st word used for check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_10(&mut self) -> H264Chkpt10W<Swreg69ReuseSpec> {
        H264Chkpt10W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 9st word used for check point used in h.264\n\n9st word used for check point used in h.264"]
    #[inline(always)]
    #[must_use]
    pub fn h264_chkpt_9(&mut self) -> H264Chkpt9W<Swreg69ReuseSpec> {
        H264Chkpt9W::new(self, 16)
    }
}
#[doc = "checkpoint 9 and 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_69_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_69_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg69ReuseSpec;
impl crate::RegisterSpec for Swreg69ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_69_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg69ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_69_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg69ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_69_REUSE to value 0"]
impl crate::Resettable for Swreg69ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
