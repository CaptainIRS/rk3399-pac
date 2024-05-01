#[doc = "Register `VID_NUM_CHUNKS` reader"]
pub type R = crate::R<VidNumChunksSpec>;
#[doc = "Register `VID_NUM_CHUNKS` writer"]
pub type W = crate::W<VidNumChunksSpec>;
#[doc = "Field `VID_NUM_CHUNKS` reader - vid_num_chunks\n\nThis register configures the number of chunks to be transmitted\n\nduring\n\na Line period (a chunk consists of a video packet and a null packet).\n\nIf set to 0 or 1, the video line is transmitted in a single packet.\n\nIf set to 1, the packet is part of a chunk, so a null packet follows it\n\nif\n\nvid_null_size > 0. Otherwise, multiple chunks are used to transmit\n\neach video line."]
pub type VidNumChunksR = crate::FieldReader<u16>;
#[doc = "Field `VID_NUM_CHUNKS` writer - vid_num_chunks\n\nThis register configures the number of chunks to be transmitted\n\nduring\n\na Line period (a chunk consists of a video packet and a null packet).\n\nIf set to 0 or 1, the video line is transmitted in a single packet.\n\nIf set to 1, the packet is part of a chunk, so a null packet follows it\n\nif\n\nvid_null_size > 0. Otherwise, multiple chunks are used to transmit\n\neach video line."]
pub type VidNumChunksW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - vid_num_chunks\n\nThis register configures the number of chunks to be transmitted\n\nduring\n\na Line period (a chunk consists of a video packet and a null packet).\n\nIf set to 0 or 1, the video line is transmitted in a single packet.\n\nIf set to 1, the packet is part of a chunk, so a null packet follows it\n\nif\n\nvid_null_size > 0. Otherwise, multiple chunks are used to transmit\n\neach video line."]
    #[inline(always)]
    pub fn vid_num_chunks(&self) -> VidNumChunksR {
        VidNumChunksR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - vid_num_chunks\n\nThis register configures the number of chunks to be transmitted\n\nduring\n\na Line period (a chunk consists of a video packet and a null packet).\n\nIf set to 0 or 1, the video line is transmitted in a single packet.\n\nIf set to 1, the packet is part of a chunk, so a null packet follows it\n\nif\n\nvid_null_size > 0. Otherwise, multiple chunks are used to transmit\n\neach video line."]
    #[inline(always)]
    #[must_use]
    pub fn vid_num_chunks(&mut self) -> VidNumChunksW<VidNumChunksSpec> {
        VidNumChunksW::new(self, 0)
    }
}
#[doc = "Number Of Chunks Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_num_chunks::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_num_chunks::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidNumChunksSpec;
impl crate::RegisterSpec for VidNumChunksSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_num_chunks::R`](R) reader structure"]
impl crate::Readable for VidNumChunksSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_num_chunks::W`](W) writer structure"]
impl crate::Writable for VidNumChunksSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_NUM_CHUNKS to value 0"]
impl crate::Resettable for VidNumChunksSpec {
    const RESET_VALUE: u32 = 0;
}
