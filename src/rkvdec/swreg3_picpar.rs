#[doc = "Register `SWREG3_PICPAR` reader"]
pub type R = crate::R<Swreg3PicparSpec>;
#[doc = "Register `SWREG3_PICPAR` writer"]
pub type W = crate::W<Swreg3PicparSpec>;
#[doc = "Field `SW_Y_HOR_VIRSTRIDE` reader - picture horizontal virtual stride\n\npicture horizontal virtual stride (the unit is 128bit)\n\nthe max is (4096x1.5 + 128) /16 = 0x188\n\nsuggest this register to config to even for advance ddr performance"]
pub type SwYHorVirstrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_Y_HOR_VIRSTRIDE` writer - picture horizontal virtual stride\n\npicture horizontal virtual stride (the unit is 128bit)\n\nthe max is (4096x1.5 + 128) /16 = 0x188\n\nsuggest this register to config to even for advance ddr performance"]
pub type SwYHorVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_SLICE_NUM_HIGHBIT` reader - the highest bit of sw_slice_num\n\nthe highest bit of sw_slice_num"]
pub type SwSliceNumHighbitR = crate::BitReader;
#[doc = "Field `SW_SLICE_NUM_HIGHBIT` writer - the highest bit of sw_slice_num\n\nthe highest bit of sw_slice_num"]
pub type SwSliceNumHighbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_UV_HOR_VIRSTRIDE` reader - uv horizontal virstride\n\npicture horizontal virtual stride (the unit is 128bit)\n\nthe max is (4096x1.5 + 128) /16 = 0x188\n\nsuggest this register to config to even for advance ddr performance"]
pub type SwUvHorVirstrideR = crate::FieldReader<u16>;
#[doc = "Field `SW_UV_HOR_VIRSTRIDE` writer - uv horizontal virstride\n\npicture horizontal virtual stride (the unit is 128bit)\n\nthe max is (4096x1.5 + 128) /16 = 0x188\n\nsuggest this register to config to even for advance ddr performance"]
pub type SwUvHorVirstrideW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_SLICE_NUM_LOWBITS` reader - slice number in a frame\n\nhevc:slice number in a frame (0~199, when it is 0, it real means 1\n\nslice in a frame)\n\njust only used for rps read.\n\n2013.11.27 change the meaning from count from 1, so it will be in\n\n1~200\n\n2013.11.30 sw_slice_num max value is change to 600, so\n\nsw_slice_num expand to 10bit\n\nh264:slice number in a frame (0~4095, when it is 1, it real means\n\n1 slice in a frame), for H264, it means sw_slice_num_lowbits\n\nvp9: no use"]
pub type SwSliceNumLowbitsR = crate::FieldReader<u16>;
#[doc = "Field `SW_SLICE_NUM_LOWBITS` writer - slice number in a frame\n\nhevc:slice number in a frame (0~199, when it is 0, it real means 1\n\nslice in a frame)\n\njust only used for rps read.\n\n2013.11.27 change the meaning from count from 1, so it will be in\n\n1~200\n\n2013.11.30 sw_slice_num max value is change to 600, so\n\nsw_slice_num expand to 10bit\n\nh264:slice number in a frame (0~4095, when it is 1, it real means\n\n1 slice in a frame), for H264, it means sw_slice_num_lowbits\n\nvp9: no use"]
pub type SwSliceNumLowbitsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:8 - picture horizontal virtual stride\n\npicture horizontal virtual stride (the unit is 128bit)\n\nthe max is (4096x1.5 + 128) /16 = 0x188\n\nsuggest this register to config to even for advance ddr performance"]
    #[inline(always)]
    pub fn sw_y_hor_virstride(&self) -> SwYHorVirstrideR {
        SwYHorVirstrideR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - the highest bit of sw_slice_num\n\nthe highest bit of sw_slice_num"]
    #[inline(always)]
    pub fn sw_slice_num_highbit(&self) -> SwSliceNumHighbitR {
        SwSliceNumHighbitR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:20 - uv horizontal virstride\n\npicture horizontal virtual stride (the unit is 128bit)\n\nthe max is (4096x1.5 + 128) /16 = 0x188\n\nsuggest this register to config to even for advance ddr performance"]
    #[inline(always)]
    pub fn sw_uv_hor_virstride(&self) -> SwUvHorVirstrideR {
        SwUvHorVirstrideR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 21:31 - slice number in a frame\n\nhevc:slice number in a frame (0~199, when it is 0, it real means 1\n\nslice in a frame)\n\njust only used for rps read.\n\n2013.11.27 change the meaning from count from 1, so it will be in\n\n1~200\n\n2013.11.30 sw_slice_num max value is change to 600, so\n\nsw_slice_num expand to 10bit\n\nh264:slice number in a frame (0~4095, when it is 1, it real means\n\n1 slice in a frame), for H264, it means sw_slice_num_lowbits\n\nvp9: no use"]
    #[inline(always)]
    pub fn sw_slice_num_lowbits(&self) -> SwSliceNumLowbitsR {
        SwSliceNumLowbitsR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - picture horizontal virtual stride\n\npicture horizontal virtual stride (the unit is 128bit)\n\nthe max is (4096x1.5 + 128) /16 = 0x188\n\nsuggest this register to config to even for advance ddr performance"]
    #[inline(always)]
    #[must_use]
    pub fn sw_y_hor_virstride(&mut self) -> SwYHorVirstrideW<Swreg3PicparSpec> {
        SwYHorVirstrideW::new(self, 0)
    }
    #[doc = "Bit 11 - the highest bit of sw_slice_num\n\nthe highest bit of sw_slice_num"]
    #[inline(always)]
    #[must_use]
    pub fn sw_slice_num_highbit(&mut self) -> SwSliceNumHighbitW<Swreg3PicparSpec> {
        SwSliceNumHighbitW::new(self, 11)
    }
    #[doc = "Bits 12:20 - uv horizontal virstride\n\npicture horizontal virtual stride (the unit is 128bit)\n\nthe max is (4096x1.5 + 128) /16 = 0x188\n\nsuggest this register to config to even for advance ddr performance"]
    #[inline(always)]
    #[must_use]
    pub fn sw_uv_hor_virstride(&mut self) -> SwUvHorVirstrideW<Swreg3PicparSpec> {
        SwUvHorVirstrideW::new(self, 12)
    }
    #[doc = "Bits 21:31 - slice number in a frame\n\nhevc:slice number in a frame (0~199, when it is 0, it real means 1\n\nslice in a frame)\n\njust only used for rps read.\n\n2013.11.27 change the meaning from count from 1, so it will be in\n\n1~200\n\n2013.11.30 sw_slice_num max value is change to 600, so\n\nsw_slice_num expand to 10bit\n\nh264:slice number in a frame (0~4095, when it is 1, it real means\n\n1 slice in a frame), for H264, it means sw_slice_num_lowbits\n\nvp9: no use"]
    #[inline(always)]
    #[must_use]
    pub fn sw_slice_num_lowbits(&mut self) -> SwSliceNumLowbitsW<Swreg3PicparSpec> {
        SwSliceNumLowbitsW::new(self, 21)
    }
}
#[doc = "picture parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg3_picpar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg3_picpar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg3PicparSpec;
impl crate::RegisterSpec for Swreg3PicparSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg3_picpar::R`](R) reader structure"]
impl crate::Readable for Swreg3PicparSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg3_picpar::W`](W) writer structure"]
impl crate::Writable for Swreg3PicparSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG3_PICPAR to value 0"]
impl crate::Resettable for Swreg3PicparSpec {
    const RESET_VALUE: u32 = 0;
}
