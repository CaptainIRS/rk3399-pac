#[doc = "Register `SWREG4_STRM_RLC_BASE` reader"]
pub type R = crate::R<Swreg4StrmRlcBaseSpec>;
#[doc = "Register `SWREG4_STRM_RLC_BASE` writer"]
pub type W = crate::W<Swreg4StrmRlcBaseSpec>;
#[doc = "Field `SW_STRM_RLC_BASE` reader - the stream or rlc data base address\n\nwhen swreg2.sw_rlc_mode =1, it is base address for rlc data\n\nwhen swreg2.sw_rlc_mode =0, it is base address for stream , after\n\na frame is decoded ready or error (stream error , time out , bus\n\nerror) , it is the last address of the stream\n\nthe address should 128bit align"]
pub type SwStrmRlcBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_STRM_RLC_BASE` writer - the stream or rlc data base address\n\nwhen swreg2.sw_rlc_mode =1, it is base address for rlc data\n\nwhen swreg2.sw_rlc_mode =0, it is base address for stream , after\n\na frame is decoded ready or error (stream error , time out , bus\n\nerror) , it is the last address of the stream\n\nthe address should 128bit align"]
pub type SwStrmRlcBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - the stream or rlc data base address\n\nwhen swreg2.sw_rlc_mode =1, it is base address for rlc data\n\nwhen swreg2.sw_rlc_mode =0, it is base address for stream , after\n\na frame is decoded ready or error (stream error , time out , bus\n\nerror) , it is the last address of the stream\n\nthe address should 128bit align"]
    #[inline(always)]
    pub fn sw_strm_rlc_base(&self) -> SwStrmRlcBaseR {
        SwStrmRlcBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - the stream or rlc data base address\n\nwhen swreg2.sw_rlc_mode =1, it is base address for rlc data\n\nwhen swreg2.sw_rlc_mode =0, it is base address for stream , after\n\na frame is decoded ready or error (stream error , time out , bus\n\nerror) , it is the last address of the stream\n\nthe address should 128bit align"]
    #[inline(always)]
    #[must_use]
    pub fn sw_strm_rlc_base(&mut self) -> SwStrmRlcBaseW<Swreg4StrmRlcBaseSpec> {
        SwStrmRlcBaseW::new(self, 4)
    }
}
#[doc = "the stream or rlc data base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg4_strm_rlc_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg4_strm_rlc_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg4StrmRlcBaseSpec;
impl crate::RegisterSpec for Swreg4StrmRlcBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg4_strm_rlc_base::R`](R) reader structure"]
impl crate::Readable for Swreg4StrmRlcBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg4_strm_rlc_base::W`](W) writer structure"]
impl crate::Writable for Swreg4StrmRlcBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG4_STRM_RLC_BASE to value 0"]
impl crate::Resettable for Swreg4StrmRlcBaseSpec {
    const RESET_VALUE: u32 = 0;
}
