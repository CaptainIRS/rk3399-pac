#[doc = "Register `USB3_PERF_CON2` reader"]
pub type R = crate::R<Usb3PerfCon2Spec>;
#[doc = "Register `USB3_PERF_CON2` writer"]
pub type W = crate::W<Usb3PerfCon2Spec>;
#[doc = "Field `USB3_SW_AR_COUNT_ID` reader - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
pub type Usb3SwArCountIdR = crate::FieldReader;
#[doc = "Field `USB3_SW_AR_COUNT_ID` writer - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
pub type Usb3SwArCountIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USB3_SW_AW_COUNT_ID` reader - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
pub type Usb3SwAwCountIdR = crate::FieldReader;
#[doc = "Field `USB3_SW_AW_COUNT_ID` writer - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
pub type Usb3SwAwCountIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
    #[inline(always)]
    pub fn usb3_sw_ar_count_id(&self) -> Usb3SwArCountIdR {
        Usb3SwArCountIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
    #[inline(always)]
    pub fn usb3_sw_aw_count_id(&self) -> Usb3SwAwCountIdR {
        Usb3SwAwCountIdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - When sw_ar_cnt_id_type=1, only count\n\nthe id designated by sw_ar_count_id"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_sw_ar_count_id(&mut self) -> Usb3SwArCountIdW<Usb3PerfCon2Spec> {
        Usb3SwArCountIdW::new(self, 0)
    }
    #[doc = "Bits 8:11 - When sw_aw_cnt_id_type=1, only count\n\nthe id designated by sw_aw_count_id"]
    #[inline(always)]
    #[must_use]
    pub fn usb3_sw_aw_count_id(&mut self) -> Usb3SwAwCountIdW<Usb3PerfCon2Spec> {
        Usb3SwAwCountIdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Usb3PerfCon2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "usb3 performance monitor control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3PerfCon2Spec;
impl crate::RegisterSpec for Usb3PerfCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_perf_con2::R`](R) reader structure"]
impl crate::Readable for Usb3PerfCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3_perf_con2::W`](W) writer structure"]
impl crate::Writable for Usb3PerfCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_PERF_CON2 to value 0"]
impl crate::Resettable for Usb3PerfCon2Spec {
    const RESET_VALUE: u32 = 0;
}
