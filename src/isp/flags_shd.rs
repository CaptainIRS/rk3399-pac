#[doc = "Register `FLAGS_SHD` reader"]
pub type R = crate::R<FlagsShdSpec>;
#[doc = "Field `ISP_ENABLE_SHD` reader - ISP enable shadow register\n\nshows, if ISP currently outputs data (1) or not (0)"]
pub type IspEnableShdR = crate::BitReader;
#[doc = "Field `INFORM_FIELD` reader - current field information (0=odd, 1=even)"]
pub type InformFieldR = crate::BitReader;
#[doc = "Field `S_DATA` reader - state of ISP input port s_data, for test purposes"]
pub type SDataR = crate::FieldReader<u16>;
#[doc = "Field `S_VSYNC` reader - state of ISP input port s_vsync, for test purposes"]
pub type SVsyncR = crate::BitReader;
#[doc = "Field `S_HSYNC` reader - state of ISP input port s_hsync, for test purposes"]
pub type SHsyncR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ISP enable shadow register\n\nshows, if ISP currently outputs data (1) or not (0)"]
    #[inline(always)]
    pub fn isp_enable_shd(&self) -> IspEnableShdR {
        IspEnableShdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - current field information (0=odd, 1=even)"]
    #[inline(always)]
    pub fn inform_field(&self) -> InformFieldR {
        InformFieldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:27 - state of ISP input port s_data, for test purposes"]
    #[inline(always)]
    pub fn s_data(&self) -> SDataR {
        SDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 30 - state of ISP input port s_vsync, for test purposes"]
    #[inline(always)]
    pub fn s_vsync(&self) -> SVsyncR {
        SVsyncR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - state of ISP input port s_hsync, for test purposes"]
    #[inline(always)]
    pub fn s_hsync(&self) -> SHsyncR {
        SHsyncR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Flags (current status) of certain signals and Shadow regs \n\n\n\nfor enable signals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flags_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlagsShdSpec;
impl crate::RegisterSpec for FlagsShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flags_shd::R`](R) reader structure"]
impl crate::Readable for FlagsShdSpec {}
#[doc = "`reset()` method sets FLAGS_SHD to value 0"]
impl crate::Resettable for FlagsShdSpec {
    const RESET_VALUE: u32 = 0;
}
