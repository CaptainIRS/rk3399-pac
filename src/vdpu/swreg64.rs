#[doc = "Register `SWREG64` reader"]
pub type R = crate::R<Swreg64Spec>;
#[doc = "Register `SWREG64` writer"]
pub type W = crate::W<Swreg64Spec>;
#[doc = "Field `SW_RLC_VLC_ST_ADR` reader - rlc or vlc mode input data start addr\n\nRLC mode:\n\nRLC data start address\n\nVLC mode:\n\nStream start address\n\nHW return value of last_byte_address by this register to tell\n\nwhere stream has been read when you get some abnormality\n\ninterrupt,may be used for debug\n\nVP7:\n\nDCT stream for MB rows 0,2n start address"]
pub type SwRlcVlcStAdrR = crate::FieldReader<u32>;
#[doc = "Field `SW_RLC_VLC_ST_ADR` writer - rlc or vlc mode input data start addr\n\nRLC mode:\n\nRLC data start address\n\nVLC mode:\n\nStream start address\n\nHW return value of last_byte_address by this register to tell\n\nwhere stream has been read when you get some abnormality\n\ninterrupt,may be used for debug\n\nVP7:\n\nDCT stream for MB rows 0,2n start address"]
pub type SwRlcVlcStAdrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - rlc or vlc mode input data start addr\n\nRLC mode:\n\nRLC data start address\n\nVLC mode:\n\nStream start address\n\nHW return value of last_byte_address by this register to tell\n\nwhere stream has been read when you get some abnormality\n\ninterrupt,may be used for debug\n\nVP7:\n\nDCT stream for MB rows 0,2n start address"]
    #[inline(always)]
    pub fn sw_rlc_vlc_st_adr(&self) -> SwRlcVlcStAdrR {
        SwRlcVlcStAdrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - rlc or vlc mode input data start addr\n\nRLC mode:\n\nRLC data start address\n\nVLC mode:\n\nStream start address\n\nHW return value of last_byte_address by this register to tell\n\nwhere stream has been read when you get some abnormality\n\ninterrupt,may be used for debug\n\nVP7:\n\nDCT stream for MB rows 0,2n start address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rlc_vlc_st_adr(&mut self) -> SwRlcVlcStAdrW<Swreg64Spec> {
        SwRlcVlcStAdrW::new(self, 2)
    }
}
#[doc = "rlc or vlc mode input data start addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg64Spec;
impl crate::RegisterSpec for Swreg64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg64::R`](R) reader structure"]
impl crate::Readable for Swreg64Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg64::W`](W) writer structure"]
impl crate::Writable for Swreg64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG64 to value 0"]
impl crate::Resettable for Swreg64Spec {
    const RESET_VALUE: u32 = 0;
}
