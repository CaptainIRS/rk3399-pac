#[doc = "Register `SWREG7_DECOUT_BASE` reader"]
pub type R = crate::R<Swreg7DecoutBaseSpec>;
#[doc = "Register `SWREG7_DECOUT_BASE` writer"]
pub type W = crate::W<Swreg7DecoutBaseSpec>;
#[doc = "Field `SW_DECOUT_BASE` reader - base address of decoder output picture addr\n\nbase address of decoder output picture\n\nthe address should be 128bit align\n\nin H264 decode format, the top field and bottom field are the same\n\naddr"]
pub type SwDecoutBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_DECOUT_BASE` writer - base address of decoder output picture addr\n\nbase address of decoder output picture\n\nthe address should be 128bit align\n\nin H264 decode format, the top field and bottom field are the same\n\naddr"]
pub type SwDecoutBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address of decoder output picture addr\n\nbase address of decoder output picture\n\nthe address should be 128bit align\n\nin H264 decode format, the top field and bottom field are the same\n\naddr"]
    #[inline(always)]
    pub fn sw_decout_base(&self) -> SwDecoutBaseR {
        SwDecoutBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address of decoder output picture addr\n\nbase address of decoder output picture\n\nthe address should be 128bit align\n\nin H264 decode format, the top field and bottom field are the same\n\naddr"]
    #[inline(always)]
    #[must_use]
    pub fn sw_decout_base(&mut self) -> SwDecoutBaseW<Swreg7DecoutBaseSpec> {
        SwDecoutBaseW::new(self, 4)
    }
}
#[doc = "base address of decoder output picture base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg7_decout_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg7_decout_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg7DecoutBaseSpec;
impl crate::RegisterSpec for Swreg7DecoutBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg7_decout_base::R`](R) reader structure"]
impl crate::Readable for Swreg7DecoutBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg7_decout_base::W`](W) writer structure"]
impl crate::Writable for Swreg7DecoutBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG7_DECOUT_BASE to value 0"]
impl crate::Resettable for Swreg7DecoutBaseSpec {
    const RESET_VALUE: u32 = 0;
}
