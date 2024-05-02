#[doc = "Register `CAC_COUNT_START` reader"]
pub type R = crate::R<CacCountStartSpec>;
#[doc = "Register `CAC_COUNT_START` writer"]
pub type W = crate::W<CacCountStartSpec>;
#[doc = "Field `h_count_start` reader - 13 bit h_count preload value (range 8191 .. 1) of the\n\nhorizontal CAC pixel counter. Before line start h_count has\n\nto be preloaded with (h_size/2 + h_center_offset), with h_size the image width and h_center_offset the horizontal distance between\n\nimage center and optical center.\n\n\n\nAfter line start the h_count decrements with every\n\npixel until a value of zero is reached for the column in the\n\noptical center. Than the h_sign bit toggles and the h_counter increments with every pixel until end of line."]
pub type HCountStartR = crate::FieldReader<u16>;
#[doc = "Field `h_count_start` writer - 13 bit h_count preload value (range 8191 .. 1) of the\n\nhorizontal CAC pixel counter. Before line start h_count has\n\nto be preloaded with (h_size/2 + h_center_offset), with h_size the image width and h_center_offset the horizontal distance between\n\nimage center and optical center.\n\n\n\nAfter line start the h_count decrements with every\n\npixel until a value of zero is reached for the column in the\n\noptical center. Than the h_sign bit toggles and the h_counter increments with every pixel until end of line."]
pub type HCountStartW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `v_count_start` reader - 13 bit v_count preload value (range 8191 ... 1) of the\n\nvertical CAC line counter. Before frame start v_count has\n\nto be preloaded with (v_size/2 + v_center_offset), with v_size the image height and v_center_offset the vertical distance between image\n\ncenter and optical center.\n\n\n\nAfter frame start the v_count decrements with every\n\nline until a value of zero is reached for the line in the\n\noptical center. Than the v_sign bit toggles and the v_counter decrements with every line until end of frame."]
pub type VCountStartR = crate::FieldReader<u16>;
#[doc = "Field `v_count_start` writer - 13 bit v_count preload value (range 8191 ... 1) of the\n\nvertical CAC line counter. Before frame start v_count has\n\nto be preloaded with (v_size/2 + v_center_offset), with v_size the image height and v_center_offset the vertical distance between image\n\ncenter and optical center.\n\n\n\nAfter frame start the v_count decrements with every\n\nline until a value of zero is reached for the line in the\n\noptical center. Than the v_sign bit toggles and the v_counter decrements with every line until end of frame."]
pub type VCountStartW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 13 bit h_count preload value (range 8191 .. 1) of the\n\nhorizontal CAC pixel counter. Before line start h_count has\n\nto be preloaded with (h_size/2 + h_center_offset), with h_size the image width and h_center_offset the horizontal distance between\n\nimage center and optical center.\n\n\n\nAfter line start the h_count decrements with every\n\npixel until a value of zero is reached for the column in the\n\noptical center. Than the h_sign bit toggles and the h_counter increments with every pixel until end of line."]
    #[inline(always)]
    pub fn h_count_start(&self) -> HCountStartR {
        HCountStartR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - 13 bit v_count preload value (range 8191 ... 1) of the\n\nvertical CAC line counter. Before frame start v_count has\n\nto be preloaded with (v_size/2 + v_center_offset), with v_size the image height and v_center_offset the vertical distance between image\n\ncenter and optical center.\n\n\n\nAfter frame start the v_count decrements with every\n\nline until a value of zero is reached for the line in the\n\noptical center. Than the v_sign bit toggles and the v_counter decrements with every line until end of frame."]
    #[inline(always)]
    pub fn v_count_start(&self) -> VCountStartR {
        VCountStartR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 13 bit h_count preload value (range 8191 .. 1) of the\n\nhorizontal CAC pixel counter. Before line start h_count has\n\nto be preloaded with (h_size/2 + h_center_offset), with h_size the image width and h_center_offset the horizontal distance between\n\nimage center and optical center.\n\n\n\nAfter line start the h_count decrements with every\n\npixel until a value of zero is reached for the column in the\n\noptical center. Than the h_sign bit toggles and the h_counter increments with every pixel until end of line."]
    #[inline(always)]
    #[must_use]
    pub fn h_count_start(&mut self) -> HCountStartW<CacCountStartSpec> {
        HCountStartW::new(self, 0)
    }
    #[doc = "Bits 16:28 - 13 bit v_count preload value (range 8191 ... 1) of the\n\nvertical CAC line counter. Before frame start v_count has\n\nto be preloaded with (v_size/2 + v_center_offset), with v_size the image height and v_center_offset the vertical distance between image\n\ncenter and optical center.\n\n\n\nAfter frame start the v_count decrements with every\n\nline until a value of zero is reached for the line in the\n\noptical center. Than the v_sign bit toggles and the v_counter decrements with every line until end of frame."]
    #[inline(always)]
    #[must_use]
    pub fn v_count_start(&mut self) -> VCountStartW<CacCountStartSpec> {
        VCountStartW::new(self, 16)
    }
}
#[doc = "Preload values for CAC pixel and line counter\n\nNote: Reset value is valid for 8192 x 8192 image resolution with centered chromatic \n\naberration (no offset from image center). \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_count_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_count_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacCountStartSpec;
impl crate::RegisterSpec for CacCountStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cac_count_start::R`](R) reader structure"]
impl crate::Readable for CacCountStartSpec {}
#[doc = "`write(|w| ..)` method takes [`cac_count_start::W`](W) writer structure"]
impl crate::Writable for CacCountStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAC_COUNT_START to value 0x1000_1000"]
impl crate::Resettable for CacCountStartSpec {
    const RESET_VALUE: u32 = 0x1000_1000;
}
